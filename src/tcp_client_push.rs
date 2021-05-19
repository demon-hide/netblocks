use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use bytes::{Bytes};

use async_stream::stream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
//use futures_util::stream::StreamExt;
pub struct TcpPush {
    //listener: TcpListener,
    //rx: mpsc::Receiver<Bytes>,
    tx: mpsc::Sender<Bytes>
}

impl TcpPush {
    pub async fn connect(addr: &str) -> Self {
        // Note that this is the Tokio TcpListener, which is fully async.
        let stream = TcpStream::connect(addr).await.expect("TcpPush, failed to connect");
        let (tx, mut rx) = mpsc::channel(10000);
        let tx3 = tx.clone();
        tokio::spawn(async move{
            let mut lines = FramedWrite::new(stream, LengthDelimitedCodec::new());
            loop {
                if let Some(msg) = rx.recv().await {
                    lines.send(msg).await;
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
    








}