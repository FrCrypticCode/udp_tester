use std::{sync::{Arc, Mutex}, collections::HashMap};
use tokio::net::UdpSocket;

pub struct SockReady{
    s:UdpSocket,
    key:u64
}
impl SockReady{
    pub async fn new(addr:&String)->Result<SockReady,String>{
        let socket = UdpSocket::bind(addr);
        match socket.await{
            Ok(x)=>{
                return Ok(SockReady{s:x,key:0})
            },
            Err(err)=>{return Err(err.to_string())}
        }
    }
   
    pub async fn rec_data(socket:&mut Self,data:&mut Arc<Mutex<HashMap<String,String>>>){
        let mut buffer = vec![0;1024];
        match socket.s.recv_buf(&mut buffer).await{
            Ok(_x)=>{
                let mut d = data.lock().unwrap();
                d.insert(socket.key.to_string(), String::from_utf8_lossy(&buffer).to_string());
                socket.key +=1;
            },
            Err(err)=>{
                println!("Une erreur est survenue lors de l'écoute : {}",err);
            }
        }
    }
}