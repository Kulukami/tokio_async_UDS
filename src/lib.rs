#![allow(unused)]

use tokio::net::{UnixStream,UnixListener};

use std::{ path::Path};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}};

pub struct Client{
    pub conn:UnixStream,
}

impl Client{
    pub async fn new<P>(path: P)->Self
    where
        P: AsRef<Path>,
    {
        let  stream = UnixStream::connect(path).await.unwrap();
        Self{
            conn:stream,
        }
    }

    pub async fn get_woker(self)->(ClientRecvier,ClientSender){
        let (rs,ss) = self.conn.into_split();
        let recv_worker = ClientRecvier::new(rs).await;
        let send_worker = ClientSender::new(ss).await;
        (recv_worker, send_worker)
    }

}


pub struct Server{
    server:UnixListener
}

impl Server {
    pub async fn new<P>(path: P)->Self
    where
        P: AsRef<Path>,
    {
        let server = UnixListener::bind(path).unwrap();
        Self{
            server:server,
        }
    }
    pub async fn accept(&self) -> (ClientRecvier,ClientSender){
        let ( stream,_addr) =  self.server.accept().await.unwrap();
        let ( rs, ss) = stream.into_split(); 
        (ClientRecvier::new(rs).await,
        ClientSender::new(ss).await,
        )
    }
}


pub struct ClientSender{
    sender:OwnedWriteHalf,
    buffer:Vec<u8>,
    buffer_max_len:usize,
    buffer_max_send_interval:i64,
    timer:i64,
}

impl ClientSender{
    pub async fn new(sender:OwnedWriteHalf)-> Self{
        Self{
            sender:sender,
            buffer:Vec::with_capacity(4096),
            buffer_max_len:512,
            buffer_max_send_interval:1024,
            timer:0,
        }
    }

    pub fn set(&mut self, buffer_max_len:usize, buffer_max_send_interval:i64){
        self.buffer_max_len = buffer_max_len;
        self.buffer_max_send_interval = buffer_max_send_interval;
    }

    pub async fn send(&mut self){
        let n = match self.sender.write(  &self.buffer).await{
            Ok(n) => {n}
            Err(ref e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                panic!("server broken pipe, client exit")
            }
            Err(e) =>{
                panic!(e)
            }
        };
        self.buffer.clear();
        if n == 0 {
            panic!("send nothing")
        }
    }

    pub fn get_buff_len(&self) -> usize{
        self.buffer.len()
    }

    pub async fn add_buff(&mut self, buf:&[u8]) {
        if buf.len() == 0{
            return
        }
        self.buffer.extend(buf);
        if self.buffer.len() > self.buffer_max_len{
            self.send().await;
            self.timer = now_monotonic();
            return 
        }
        let now = now_monotonic();
        if now - self.timer >= self.buffer_max_send_interval{
            self.timer = now;
            self.send().await;
        }
    }
}

pub struct ClientRecvier{
    recver:OwnedReadHalf,
}

impl ClientRecvier{
    pub async fn new(recver:OwnedReadHalf)-> Self{
        Self{
            recver:recver
        }
    }
    pub async fn recive(&mut self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::with_capacity(10240);
        let n = self.recver.read_buf(&mut buf).await.unwrap();
        match n {
            0 => {
                panic!()
            }
            _ =>{
                buf
            }
        }
    }
}


pub fn now_monotonic() -> i64{
    let mut time = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let ret = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, &mut time) };
    assert!(ret == 0);
    time.tv_sec
}


#[cfg(test)]
mod tests {
    use std::{mem::swap, thread, time::Duration};
    use crate::now_monotonic;
    #[test]
    fn it_works() {
        let a =  now_monotonic();
        println!("{}",a);
        thread::sleep(Duration::from_millis(1000));
        let a =  now_monotonic();
        println!("{}",a);
        thread::sleep(Duration::from_millis(1000));
        let a =  now_monotonic();
        println!("{}",a);
        //assert_eq!(2 + 2, 4);
    }
}

