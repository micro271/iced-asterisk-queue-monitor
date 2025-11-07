use std::{mem::transmute_copy, task::Poll};

use futures::{FutureExt, StreamExt};
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadBuf}, net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}};

use crate::asterisk::entities::{Entry, Params, member::*};


pub struct EventHandler {
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
    buffer: Vec<u8>,
    pos: usize,
    find_until: usize,
}

impl EventHandler {
    pub fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        Self {
            reader,
            writer,
            pos: 0,
            buffer: Vec::new(),
            find_until: 0,
        }
    }
    
    pub async fn login(&mut self, user: &str, secret: &str) {
        println!("1");
        self.writer.write_all(format!("Action: Login\r\nUsername: {user}\r\nSecret: {secret}\r\n\r\n").as_bytes()).await.unwrap();
        //let tmp = self.next().await;
    }

    pub async fn join_queue(&mut self) {
        self.writer.write_all(b"Action: Events\r\nEventMask: queue\r\n\r\n").await.unwrap();
    }
}

impl futures::stream::Stream for EventHandler {
    type Item = Result<QueueEvent, ()>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            println!("{}", this.pos);
            this.pos += 1;
            let mut buf = [0u8; 1024];
            let n = match futures::ready!(this.reader.read(&mut buf).boxed().poll_unpin(cx)) {
                Ok(n ) => n,
                Err(_) => break Poll::Ready(None),
            };

            this.buffer.extend_from_slice(&buf[..n]);
            

            if let Some(e) = this.buffer[this.find_until..].windows(4).position(|x| x == b"\r\n\r\n") {
                println!("{:?}", std::str::from_utf8(&this.buffer[..this.find_until + e + 4]));
                //let resp = QueueEvent::try_from(&this.buffer[..(this.find_until+e)]);
                this.buffer.drain(..(this.find_until + e + 4));
                if n == 0 && this.buffer.len() != 0 {
                    this.buffer.clear();
                }
                this.find_until = 0;
                break Poll::Ready(Some(Ok(QueueEvent::CallerAbandon)));
            } else if n == 0 {
                this.buffer.clear();
                break Poll::Ready(Some(Err(())));
            } else {
                this.find_until += n.saturating_sub(4);
            }
        }

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
