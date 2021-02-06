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
}

impl ClientSender{
    pub async fn new(sender:OwnedWriteHalf,)-> Self{
        Self{
            sender:sender
        }
    }
    pub async fn send(&mut self,buf:&[u8]){
        //let mut buf:Vec<u8> = Vec::with_capacity(10240);
        let n = match self.sender.write(  buf).await{
            Ok(n) => {n}
            Err(ref e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                panic!()
            }
            Err(e) =>{
                panic!(e)
            }
        };
        if n == 0 {
            panic!("send nothing")
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

