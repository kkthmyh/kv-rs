use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_rs::{CommandRequest, CommandResponse};
use tokio::net::TcpListener;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); let addr = "127.0.0.1:8888"; 
    let listener = TcpListener::bind(addr).await?; 
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?; 
        info!("Client {:?} connected", addr);
        tokio::spawn(async move {
            let mut stream  = AsyncProstStream::<_,CommandRequest,CommandResponse,_>::from(stream).for_async();
            while let Some(Ok(data)) = stream.next().await {
                info!("Got a new command: {:?}", data);
                // 响应client
                let mut resp = CommandResponse::default(); 
                resp.status = 404; 
                resp.message = "Not found".to_string(); 
                stream.send(resp).await.unwrap();
                info!("Send a response: {:?}", 404);
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
