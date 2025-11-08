use std::{mem::transmute_copy, task::Poll};

use bytes::{Buf, Bytes, BytesMut};
use futures::FutureExt;
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadBuf}, net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}};

use crate::asterisk::entities::{Entry, Params, member::*};


pub struct EventHandler {
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
    buffer: BytesMut,
    state: State,
}

impl EventHandler {
    pub fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        Self {
            reader,
            writer,
            buffer: BytesMut::new(),
            state: State::State0,
        }
    }
    
    pub async fn login(&mut self, user: &str, secret: &str) {
        self.writer.write_all(format!("Action: Login\r\nUsername: {user}\r\nSecret: {secret}\r\n\r\n").as_bytes()).await.unwrap();
    }

    pub async fn join_queue(&mut self) {
        self.writer.write_all(b"Action: Events\r\nEventMask: queue,agent\r\n\r\n").await.unwrap();
    }
}

impl futures::stream::Stream for EventHandler {
    type Item = Result<QueueEvent, ()>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            match this.state.clone() {
                State::Done => { return Poll::Ready(None); }
                State::State0 => {
                    println!("entramos");
                    this.state = State::Read;
                }
                State::CheckToProcess{check } => {
                    if let Some(n) = this.buffer.windows(4).rposition(|x| x == b"\r\n\r\n") {
                        this.state = State::Process(this.buffer.split_to(n+4), n);
                    } else if !check.is_to_continue() {
                        this.state = State::Done;
                    } else {
                        this.state = State::Read;
                    }
                }
                State::Read => {
                    match futures::ready!(this.reader.read(&mut this.buffer).boxed().poll_unpin(cx)) {
                        Ok(n ) => {
                            if n == 0 {
                                this.state = State::EOF
                            } else {
                                println!("bytes leidos {n}");
                                this.state = State::CheckToProcess{check: InnerStateCheckToProcess::ToContinue}
                            }
                        },
                        Err(er) => {
                            println!("{er}");
                            return Poll::Ready(None)
                        },
                    }
                },
                State::Process(mut bytes, n) => {
                    println!("{:?}", std::str::from_utf8(bytes.split_to(n).as_ref()));
                    if !bytes.is_empty() {
                        this.state = State::CheckToProcess{check: InnerStateCheckToProcess::ToContinue};
                        return Poll::Ready(Some(Ok(QueueEvent::CallerAbandon)));
                    } else {
                        this.state = State::Read
                    }
                }
                State::EOF => {
                    println!("FIN");
                    this.state = State::CheckToProcess{check: InnerStateCheckToProcess::ToFinish};
                }
            }
            println!("{:?}", this.state);
        }

    }
}

#[derive(Debug, Clone)]
enum State {
    State0,
    Read,
    CheckToProcess{check: InnerStateCheckToProcess},
    Process(BytesMut, usize),
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

fn from(vec: &mut [u8]) -> QueueEvent {
    todo!()
}



#[derive(Debug)]
pub enum QueueEvent {
    Params(Params),
    Entry(Entry),
    StatusComplete,
    CallerJoin,
    CallerLeave,
    CallerAbandon,
    CallerReconnect,
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

impl std::fmt::Display for QueueEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueEvent::Params(e) => write!(f, "QueueParams({e:?})"),
            QueueEvent::Entry(e) => write!(f, "QueueEntry({e:?})"),
            QueueEvent::StatusComplete => write!(f, "QueueStatusComplete"),
            QueueEvent::CallerJoin => write!(f, "QueueCallerJoin"),
            QueueEvent::CallerLeave => write!(f, "QueueCallerLeave"),
            QueueEvent::CallerAbandon => write!(f, "QueueCallerAbandon"),
            QueueEvent::CallerReconnect => write!(f, "QueueCallerReconnect"),
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

impl TryFrom<&[u8]> for QueueEvent {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        QueueEvent::try_from(std::str::from_utf8(value).unwrap())
    }
}

impl TryFrom<&str> for QueueEvent {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        /* 
        match value {
            "QueueParams" => Ok(QueueEvent::Params),
            "QueueEntry" => Ok(QueueEvent::Entry()),
            "QueueStatusComplete" => Ok(QueueEvent::StatusComplete),
            "QueueCallerJoin" => Ok(QueueEvent::CallerJoin),
            "QueueCallerLeave" => Ok(QueueEvent::CallerLeave),
            "QueueCallerAbandon" => Ok(QueueEvent::CallerAbandon),
            "QueueCallerReconnect" => Ok(QueueEvent::CallerReconnect),
            "QueueMemberPaused" => Ok(QueueEvent::MemberPaused),
            "QueueMemberStatus" => Ok(QueueEvent::MemberStatus),
            "QueueMemberAdded" => Ok(QueueEvent::MemberAdded),
            "QueueMemberCaller" => Ok(QueueEvent::MemberCaller),
            "QueueMemberConnect" => Ok(QueueEvent::MemberConnect),
            "QueueMemberRemoved" => Ok(QueueEvent::MemberRemoved),
            "QueueMemberComplete" => Ok(QueueEvent::MemberComplete),
            "QueueMemberRingnoanswer" => Ok(QueueEvent::MemberRingnoanswer),
            "QueueMemberBusy" => Ok(QueueEvent::MemberBusy),
            _ => Err(String::new())
        }
        */
        todo!()
    }
}
