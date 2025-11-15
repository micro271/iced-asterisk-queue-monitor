use std::{collections::HashMap, pin::Pin, task::Poll};

use futures::FutureExt;
use tokio::{
    io::{AsyncReadExt, AsyncWrite},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use crate::{asterisk::entities::{
    Entry, Params, ResponseAmi, StatusComplete,
    agent::{AgentComplete, AgentConnect, AgentDump, AgentRingNoAnswer, AgenteCalled},
    caller::{Caller, TypeCallerEvent},
    member::*,
}, io::writer::BufWriter};

pub struct EventHandler {
    reader: OwnedReadHalf,
    writer: BufWriter<OwnedWriteHalf>,
    buffer: Vec<u8>,
    processed: usize,
    state: State,
    username: String,
    secret: String,
}

impl EventHandler {
    pub fn new(stream: TcpStream, username: String, secret: String) -> Self {
        let (reader, writer) = stream.into_split();
        Self {
            reader,
            writer: BufWriter::new(writer),
            buffer: Vec::new(),
            processed: 0,
            state: State::State0Login,
            username,
            secret,
        }
    }

    pub fn login(&self) -> String {
        format!(
            "Action: Login\r\nUsername: {}\r\nSecret: {}\r\n\r\n",
            self.username, self.secret
        )
    }

    pub fn event(&self) -> &'static [u8] {
        b"Action: Events\r\nEventMask: queue,agent\r\n\r\n"
    }

    pub fn info_queue(&self) -> &'static [u8] {
        b"Action: QueueStatus\r\n\r\n"
    }
}

impl futures::stream::Stream for EventHandler {
    type Item = Result<AmiMessage, ()>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            match this.state.clone() {
                State::Write => {
                    let pin = Pin::new(&mut this.writer);
                    if let Err(er) = futures::ready!(pin.poll_flush(cx)) {
                        println!("{}", er);
                    }

                    this.state = State::Read;
                }
                State::State0Login => {
                    this.writer.to_write(this.login().as_bytes());
                    this.state = State::Write;
                }
                State::State1Subscriber => {
                    this.writer.to_write(this.event());
                    this.state = State::Write;
                }
                State::State2Data => {
                    this.writer.to_write(this.info_queue());
                    this.state = State::Write;
                }
                State::Done => return Poll::Ready(None),
                State::CheckToProcess { check } => {
                    if let Some(pos) = this.buffer[this.processed..]
                        .windows(4)
                        .position(|x| x == b"\r\n\r\n")
                    {
                        this.state = State::Process;
                        this.processed += pos + 3;
                    } else if !check.is_to_continue() {
                        this.state = State::Done;
                    } else {
                        this.state = State::Read;
                        this.processed = this.buffer.len().saturating_sub(4);
                    }
                }
                State::Read => {
                    let mut buf = [0u8; 1024];
                    let n = match futures::ready!(this.reader.read(&mut buf).boxed().poll_unpin(cx))
                    {
                        Ok(n) => n,
                        Err(er) => {
                            println!("{er}");
                            return Poll::Ready(None);
                        }
                    };
                    if n == 0 {
                        this.state = State::Eof
                    } else {
                        println!("bytes leidos {n}");
                        this.buffer.extend_from_slice(&buf[..n]);
                        this.state = State::CheckToProcess {
                            check: InnerStateCheckToProcess::ToContinue,
                        }
                    }
                }
                State::Process => {
                    let data = std::str::from_utf8(&this.buffer[..=this.processed]).unwrap();
                    let data = data.trim_end().to_string();
                    println!("\n\n{data}");
                    let tmp = AmiMessage::try_from(&data[..]);
                    println!("\n{tmp:?}");
                    this.buffer.drain(..=this.processed);
                    this.processed = 0;
                    if data.ends_with("Authentication accepted") {
                        this.state = State::State1Subscriber;
                        continue;
                    }
                    if data.ends_with("Events: On") {
                        this.state = State::State2Data;
                        continue;
                    }
                    if !this.buffer.is_empty() {
                        this.state = State::CheckToProcess {
                            check: InnerStateCheckToProcess::ToContinue,
                        };
                        return Poll::Ready(Some(tmp));
                    } else {
                        this.state = State::Read
                    }
                }
                State::Eof => {
                    println!("FIN");
                    this.state = State::CheckToProcess {
                        check: InnerStateCheckToProcess::ToFinish,
                    };
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    State0Login,
    State1Subscriber,
    Write,
    State2Data,
    Read,
    CheckToProcess { check: InnerStateCheckToProcess },
    Process,
    Done,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
enum InnerStateCheckToProcess {
    ToContinue,
    ToFinish,
}

impl InnerStateCheckToProcess {
    fn is_to_continue(&self) -> bool {
        matches!(self, InnerStateCheckToProcess::ToContinue)
    }
}

#[derive(Debug)]
pub enum AmiMessage {
    Response(ResponseAmi),

    Params(Params),
    Entry(Entry),
    StatusComplete(StatusComplete),
    CallerJoin(Caller),
    CallerLeave(Caller),
    CallerAbandon(Caller),

    Member(Member),
    MemberStatus(Member),
    MemberPaused(Member),
    MemberAdded(Member),
    MemberRemoved(Member),
    MemberRingninuse(MemberRingninuse),

    AgentCalled(AgenteCalled),
    AgentConnect(AgentConnect),
    AgentComplete(AgentComplete),

    AgentRingNoAnswer(AgentRingNoAnswer), // si AMI tiene campos asociados, hacer struct
    AgentDump(AgentDump),
    None,
}

impl TryFrom<&str> for AmiMessage {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut map = EventGenMap::gen_map(value);

        if map.contains_key("Response") {
            if map.contains_key("Events") {
                return Ok(AmiMessage::Response(ResponseAmi::parse_from_map(map).r#type(super::entities::ResponseAmyType::Action)));
            } else {
                return Ok(AmiMessage::Response(ResponseAmi::parse_from_map(map).r#type(super::entities::ResponseAmyType::Login)));
            }
        }

        match map.remove("Event").unwrap_or_default() {
            "QueueStatusComplete" => Ok(AmiMessage::StatusComplete(
                StatusComplete::parse_from_map(map),
            )),
            "QueueMemberStatus" => Ok(Self::MemberStatus(Member::parse_from_map(map))),
            "QueueParams" => Ok(Self::Params(Params::parse_from_map(map))),
            "AgentCalled" => Ok(Self::AgentCalled(AgenteCalled::parse_from_map(map))),
            "AgentConnect" => Ok(Self::AgentConnect(AgentConnect::parse_from_map(map))),
            "AgentComplete" => Ok(Self::AgentComplete(AgentComplete::parse_from_map(map))),
            "AgentRingNoAnswer" => Ok(Self::AgentRingNoAnswer(AgentRingNoAnswer::parse_from_map(
                map,
            ))),
            "QueueMember" => Ok(Self::Member(Member::parse_from_map(map))),
            "AgentDump" => Ok(Self::AgentDump(AgentDump::parse_from_map(map))),
            "QueueMemberPaused" => Ok(Self::MemberPaused(Member::parse_from_map(map))),
            "QueueMemberAdded" => Ok(Self::MemberAdded(Member::parse_from_map(map))),
            "QueueMemberRemoved" => Ok(Self::MemberRemoved(Member::parse_from_map(map))),
            "MemberRingninuse" => Ok(Self::MemberRingninuse(MemberRingninuse::parse_from_map(
                map,
            ))),
            "QueueEntry" => Ok(Self::Entry(Entry::parse_from_map(map))),
            "QueueCallerJoin" => Ok(Self::CallerJoin(
                Caller::parse_from_map(map).r#type(TypeCallerEvent::Join),
            )),
            "QueueCallerLeave" => Ok(Self::CallerLeave(
                Caller::parse_from_map(map).r#type(TypeCallerEvent::Leave),
            )),
            "QueueCallerAbandon" => Ok(Self::CallerAbandon(
                Caller::parse_from_map(map).r#type(TypeCallerEvent::Abandon),
            )),
            _ => Ok(Self::None),
        }
    }
}

impl std::fmt::Display for AmiMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmiMessage::None => write!(f, "None"),
            AmiMessage::Response(_) => write!(f, "MemberRingninuse"),
            AmiMessage::MemberRingninuse(_) => write!(f, "MemberRingninuse"),
            AmiMessage::Member(_) => write!(f, "QueueMember"),
            AmiMessage::Params(_) => write!(f, "QueueParams"),
            AmiMessage::Entry(_) => write!(f, "QueueEntry"),
            AmiMessage::StatusComplete(_) => write!(f, "QueueStatusComplete"),
            AmiMessage::CallerJoin(_) => write!(f, "QueueCallerJoin"),
            AmiMessage::CallerLeave(_) => write!(f, "QueueCallerLeave"),
            AmiMessage::CallerAbandon(_) => write!(f, "QueueCallerAbandon"),
            AmiMessage::MemberPaused(_) => write!(f, "QueueMemberPaused"),
            AmiMessage::MemberStatus(_) => write!(f, "QueueMemberStatus"),
            AmiMessage::MemberAdded(_) => write!(f, "QueueMemberAdded"),
            AmiMessage::AgentCalled(_) => write!(f, "AgentCalled"),
            AmiMessage::AgentConnect(_) => write!(f, "AgentConnect"),
            AmiMessage::MemberRemoved(_) => write!(f, "QueueMemberRemoved"),
            AmiMessage::AgentComplete(_) => write!(f, "AgentComplete"),
            AmiMessage::AgentRingNoAnswer(_) => write!(f, "AgentRingNoAnswer"),
            AmiMessage::AgentDump(_) => write!(f, "AgentDump"),
        }
    }
}

pub struct EventGenMap;

impl EventGenMap {
    pub fn gen_map(data: &str) -> HashMap<&str, &str> {
        data.lines()
            .map(|x| {
                let (key, value) = x.split_once(':').unwrap_or_default();
                (key.trim(), value.trim())
            })
            .collect()
    }
}

pub trait ParserEvent {
    fn parse_from_map(data: HashMap<&str, &str>) -> Self
    where
        Self: Sized;

    fn parser(data: &str) -> Self
    where
        Self: Sized,
    {
        Self::parse_from_map(EventGenMap::gen_map(data))
    }
}
