use futures::future::join_all;
use std::{env::args, fs, time::Instant, ops::Add};
use tokio::{net::TcpStream, sync::watch::{Receiver, channel}};

async fn tcp_scan(rx: Receiver<Option<String>>) -> Vec<u8> {
    let mut handles = vec![];
    for i in 1..=65535 {
        let mut rx = rx.clone();
        let handle = async move {
            rx.changed().await.unwrap();
            let addr = format!("{}:{i}", *rx.borrow().as_ref().unwrap());
            match TcpStream::connect(&addr).await {
                Ok(_) => addr.add(" 打开了\n").as_bytes().to_vec(),
                Err(_) => addr.add(" 关闭了\n").as_bytes().to_vec(),
            }
        };
        handles.push(handle);
    }
    join_all(handles)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
}

async fn exe_tcp_scan() -> Vec<u8> {
    let (tx, rx) = channel(None);
    let res = tcp_scan(rx);
    let addr = args().into_iter().nth(1);
    tx.send(addr).unwrap();
    res.await
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    fs::write("result.txt", exe_tcp_scan().await).unwrap();
    let elapsed = start.elapsed().as_secs();
    println!("Execution Time: {} Secs.", elapsed);
}