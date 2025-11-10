use futures::StreamExt;
use tokio::{
    net::TcpStream,
};

use crate::asterisk::event::EventHandler;

pub mod entities;
pub mod event;

pub struct Alma;

impl Alma {
    pub async fn run(socket: String, user: String, secret: String) {
        let stream = TcpStream::connect(socket).await.unwrap();

        let mut tmp = EventHandler::new(stream, user, secret);

        loop {
            let tmp = tmp.next().await;
            if tmp.is_none() {
                break;
            }
            println!("{tmp:?}");
        }
    }
}
