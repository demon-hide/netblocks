use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, FramedWrite, LengthDelimitedCodec};

use futures::{Future, SinkExt, future::Select};
use std::{collections::HashMap, process::Output};
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use bytes::{Bytes};

use async_stream::stream;
use futures_core;
use futures_core::stream::Stream;
use futures_util::pin_mut;
use tokio_core::reactor::Core;
use std::thread;
//use futures_util::stream::StreamExt;
pub struct TcpPush {
    //listener: TcpListener,
    //rx: mpsc::Receiver<Bytes>,
    tx: mpsc::Sender<Bytes>,
    handle: thread::JoinHandle<(tokio::task::JoinHandle<()>)>
}

impl TcpPush {
    pub async fn bind(addr: &str) -> Self {
        // Note that this is the Tokio TcpListener, which is fully async.
        let listener = TcpListener::bind(&addr).await.expect("TcpPull, failed to bind");
        let (tx, mut rx) = mpsc::channel(10000);
        let tx3 = tx.clone();
        let rt = tokio::runtime::Handle::current();
        let worker = move|| {
            rt.spawn(async move {
                loop {
                    // Asynchronously wait for an inbound TcpStream.
                    let (stream, addr) = listener.accept().await.expect("TcpPull, failure accepting connection");
                    let tx2 = tx3.clone();    
                    // Spawn our handler to be run asynchronously.
                    println!("came in guest");
                    //tokio::spawn(async move {
                        let mut lines = FramedWrite::new(stream, LengthDelimitedCodec::new());

                      
                        loop {
                            if let Some(msg) = rx.recv().await {
                                lines.send(msg).await;
                            }
                        }
            //});
                }
            })

    };
        let handle= thread::spawn(worker);

        Self {
           // listener,
            handle:handle,
            //rx,
            tx: tx.clone()
        }
    }

    pub async fn send(&self, message: Bytes)  {
        
        self.tx.send(message).await;
        
    }
}






