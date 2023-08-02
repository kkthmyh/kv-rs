use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_rs::{CommandRequest, CommandResponse, Value};
use tokio::net::TcpStream;
use tracing::info;
/**
 * client 代码
 */
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9528";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("test","rust","hello".into());
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }  
    Ok(())
}
