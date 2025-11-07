use std::net::SocketAddr;

use futures::StreamExt;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream};

use crate::asterisk::event::{EventHandler, QueueEvent};

pub mod entities;
pub mod event;

pub struct Alma;



impl Alma {
    pub async fn run(socket: String,user: String, secret: String) {
        let stream = TcpStream::connect(socket).await.unwrap();
        
        let mut tmp = EventHandler::new(stream);
        tmp.login("callcenter", "test").await;
        tmp.join_queue().await;
        loop {
            let tmp = tmp.next().await;
            //println!("{tmp:?}");
        }
    }
}