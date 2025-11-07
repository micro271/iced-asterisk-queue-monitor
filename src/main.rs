pub mod app;
pub mod asterisk;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream};

use crate::asterisk::Alma;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    _ = dotenv::dotenv();


    let user = std::env::var("USER").expect("Secret not found");
    let secret = std::env::var("SECRET").expect("Secret not found");
    Alma::run("192.168.10.100:5038".to_string(), user, secret).await;
    Ok(())
}
