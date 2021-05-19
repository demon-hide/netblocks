use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};
use tokio_util::codec::BytesCodec;
use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use bytes::{Bytes};
use tokio_util::udp::UdpFramed;
use async_stream::stream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
//use futures_util::stream::StreamExt;
pub struct UdpPush {
    //listener: TcpListener,
    //rx: mpsc::Receiver<Bytes>,
    tx: mpsc::Sender<Bytes>
}

impl UdpPush {
    pub async fn connect(addr: &str) -> Self {
        // Note that this is the Tokio TcpListener, which is fully async.
        let socket = UdpSocket::bind("127.0.0.1:0").await.expect("TcpPush, failed to connect");
        let (tx, mut rx) = mpsc::channel::<Bytes>(100);
       // let tx3 = tx.clone();
       let addr2= addr.parse().expect("Incorrect ip addr");
        tokio::spawn(async move{
            let mut frames = UdpFramed::new(socket, BytesCodec::new());
             loop {
                if let Some(msg) = rx.recv().await {
                    //let imsg: Bytes = msg;
                      frames.send((msg,addr2)).await;
                }
            }
        });

    Self {
        // listener,
         //rx,
         tx: tx.clone()
     }
}

pub async fn send(&self, message: Bytes)  {
        
        self.tx.send(message).await;
        
    }
    
    pub async fn close(&self, message: Bytes)  {
        
        //self.tx.close().await;
        
    }







}