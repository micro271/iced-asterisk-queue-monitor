use std::{collections::HashMap, mem::transmute_copy, task::Poll};

use futures::FutureExt;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadBuf},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use crate::asterisk::entities::{Entry, Params, caller::Caller, member::*};

pub struct EventHandler {
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
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
            writer,
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
    type Item = Result<QueueEvent, ()>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            match this.state.clone() {
                State::State0Login => {
                    match futures::ready!(
                        this.writer
                            .write(this.login().as_bytes())
                            .boxed()
                            .poll_unpin(cx)
                    ) {
                        Ok(e) => {
                            println!("Bytes written {e}");
                            this.state = State::Read;
                        }
                        Err(_) => return Poll::Ready(None),
                    }
                }
                State::State1Subscriber => {
                    match futures::ready!(this.writer.write(this.event()).boxed().poll_unpin(cx)) {
                        Ok(e) => {
                            println!("Subscriber - bytes escritos: {e}");
                            this.state = State::Read;
                        }
                        Err(_) => return Poll::Ready(None),
                    }
                }
                State::State2Data => {
                    match futures::ready!(
                        this.writer.write(this.info_queue()).boxed().poll_unpin(cx)
                    ) {
                        Ok(e) => {
                            println!("Subscriber - bytes escritos: {e}");
                            this.state = State::Read;
                        }
                        Err(_) => return Poll::Ready(None),
                    }
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
                        this.state = State::EOF
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
                    let tmp = QueueEvent::try_from(&data[..]);
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
                State::EOF => {
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
    State2Data,
    Read,
    CheckToProcess { check: InnerStateCheckToProcess },
    Process,
    Done,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
enum InnerStateCheckToProcess {
    ToContinue,
    ToFinish,
}

impl InnerStateCheckToProcess {
    fn is_to_continue(self) -> bool {
        matches!(self, InnerStateCheckToProcess::ToContinue)
    }
}

#[derive(Debug)]
pub enum QueueEvent {
    Params(Params),
    Entry(Entry),
    StatusComplete,
    CallerJoin(Caller),
    CallerLeave(Caller),
    CallerAbandon(Caller),
    CallerReconnect(Caller),
    Member(MemberStatus),
    MemberStatus(MemberStatus),
    MemberPaused(MemberPaused),
    MemberAdded(MemberAdded),
    MemberCaller(MemberCaller),
    MemberConnect(MemberConnect),
    MemberRemoved(MemberRemoved),
    MemberComplete(MemberComplete),
    MemberRingnoanswer, // si AMI tiene campos asociados, hacer struct
    MemberBusy,
}

impl TryFrom<&str> for QueueEvent {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut map = EventGenMap::gen_map(value);

        match map.remove("Event").unwrap_or_default() {
            "QueueMemberStatus" => Ok(Self::MemberStatus(MemberStatus::parse_from_map(map))),
            "QueueMember" => Ok(Self::Member(MemberStatus::parse_from_map(map))),
            "QueueParams" => Ok(Self::Params(Params::parse_from_map(map))),
            /*
            "QueueEntry" => Ok(Self::Entry),
            "QueueStatusComplete" => Ok(Self::StatusComplete),
            "QueueCallerJoin" => Ok(Self::CallerJoin),
            "QueueCallerLeave" => Ok(Self::CallerLeave),
            "QueueCallerAbandon" => Ok(Self::CallerAbandon),
            "QueueCallerReconnect" => Ok(Self::CallerReconnect),
            "QueueMemberPaused" => Ok(Self::MemberPaused),
            "QueueMemberAdded" => Ok(Self::MemberAdded),
            "QueueMemberCaller" => Ok(Self::MemberCaller),
            "QueueMemberConnect" => Ok(Self::MemberConnect),
            "QueueMemberRemoved" => Ok(Self::MemberRemoved),
            "QueueMemberComplete" => Ok(Self::MemberComplete),
            "QueueMemberRingnoanswer" => Ok(Self::MemberRingnoanswer),
            "QueueMemberBusy" => Ok(Self::MemberBusy),*/
            _ => Ok(Self::MemberBusy),
        }
    }
}

impl std::fmt::Display for QueueEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueEvent::Member(_) => write!(f, "QueueMember"),
            QueueEvent::Params(_) => write!(f, "QueueParams"),
            QueueEvent::Entry(_) => write!(f, "QueueEntry"),
            QueueEvent::StatusComplete => write!(f, "QueueStatusComplete"),
            QueueEvent::CallerJoin(_) => write!(f, "QueueCallerJoin"),
            QueueEvent::CallerLeave(_) => write!(f, "QueueCallerLeave"),
            QueueEvent::CallerAbandon(_) => write!(f, "QueueCallerAbandon"),
            QueueEvent::CallerReconnect(_) => write!(f, "QueueCallerReconnect"),
            QueueEvent::MemberPaused(_) => write!(f, "QueueMemberPaused"),
            QueueEvent::MemberStatus(_) => write!(f, "QueueMemberStatus"),
            QueueEvent::MemberAdded(_) => write!(f, "QueueMemberAdded"),
            QueueEvent::MemberCaller(_) => write!(f, "QueueMemberCaller"),
            QueueEvent::MemberConnect(_) => write!(f, "QueueMemberConnect"),
            QueueEvent::MemberRemoved(_) => write!(f, "QueueMemberRemoved"),
            QueueEvent::MemberComplete(_) => write!(f, "QueueMemberComplete"),
            QueueEvent::MemberRingnoanswer => write!(f, "QueueMemberRingnoanswer"),
            QueueEvent::MemberBusy => write!(f, "QueueMemberBusy"),
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
