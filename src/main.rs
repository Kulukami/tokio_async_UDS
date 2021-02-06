use std::{str::from_utf8};
use std::time::Duration;
use std::thread;
use tokio::runtime::Handle;

pub const SOCKPATH:&str="/tmp/rust/p.sock";


#[tokio::main]
async fn main() {
    // This is running on a core thread.
    let  c = async_client::Client::new(SOCKPATH).await;
    let (mut r,mut s) = c.get_woker().await;

    let handle = Handle::current();
    thread::spawn(move || {
        handle.spawn(async move {
            loop {
                s.send(b"client send test").await;
                //buf.clear();
                //buf.extend(pids.as_bytes());
                thread::sleep(Duration::from_millis(125));
            }
        });

    });
    loop {
        let data = r.recive().await;
        println!("recive {:x?}",from_utf8(&data).unwrap());
        //thread::sleep(Duration::from_millis(125));
    }
}