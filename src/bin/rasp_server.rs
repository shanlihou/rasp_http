use async_std::net::{TcpStream, TcpListener};
use async_std::prelude::*;
use async_std::task;
use http_types::{Response, StatusCode};

#[async_std::main]
async fn main() -> http_types::Result<()> {
    // 建立TCP连接并创建一个URL
    let listener = TcpListener::bind(("0.0.0.0", 48080)).await?;
    let addr = format!("http://{}", listener.local_addr()?);
    println!("listening on {}", addr);

    // 对每个TCP连接，spawn一个任务并用accept去处理
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        if let Ok(_stream) = stream {
            task::spawn(async {
                async_h1::accept(_stream, my_accept).await;
            });
        }
    }
    Ok(())
}

async fn my_accept(req: http_types::Request) -> http_types::Result<Response>{
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain");
    res.set_body("Hello");
    Ok(res)
}