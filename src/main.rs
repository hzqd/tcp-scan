use futures::{future::join_all, TryFutureExt};
use std::{env::args, fs, time::Instant, vec};
use tokio::net::TcpStream;

async fn tcp_scan() -> Vec<u8> {
    let mut handles = vec![];
    let addr = &args().into_iter().collect::<Vec<_>>()[1];
    for i in 1..=65535 {
        let handle = TcpStream::connect(format!("{addr}:{i}")).map_ok_or_else(
            move |_| format!("{addr}:{i} 关闭了\n"),
            move |_| format!("{addr}:{i} 打开了！！！\n"),
        );
        handles.push(handle);
    }
    join_all(handles)
        .await
        .into_iter()
        .flat_map(|s| s.as_bytes().to_vec())
        .collect()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    fs::write("result.txt", tcp_scan().await).unwrap();
    let elapsed = start.elapsed().as_secs();
    println!("Execution Time: {} Secs.", elapsed);
}
