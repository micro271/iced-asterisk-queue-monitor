pub mod app;
pub mod asterisk;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::asterisk::Alma;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    _ = dotenv::dotenv();

    let user = std::env::var("USERNAME").expect("Secret not found");
    let secret = std::env::var("SECRET").expect("Secret not found");
    let socket_ami = std::env::var("AMI").expect("Socket AMI not found");

    Alma::run(socket_ami, user, secret).await;
    Ok(())
}
