use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::codec::{Framed, FramedRead, LengthDelimitedCodec};
use tokio_util::udp::UdpFramed;
//use futures::{Future, SinkExt, , future::Select};
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
pub struct UdpPull {
    //listener: TcpListener,
    rx: mpsc::Receiver<Bytes>,
    tx: mpsc::Sender<Bytes>,
    handle: thread::JoinHandle<(tokio::task::JoinHandle<()>)>
}

impl UdpPull {
    pub async fn bind(addr: &str) -> Self {
        // Note that this is the Tokio TcpListener, which is fully async.
        let socket = UdpSocket::bind(&addr).await.expect("UdpPull, failed to bind");
        let (tx, rx) = mpsc::channel(10000);
        let tx3 = tx.clone();
        let rt = tokio::runtime::Handle::current();
        let worker = move|| {
            rt.spawn(async move {
               // loop {
                    // Asynchronously wait for an inbound TcpStream.
                   // let (stream, addr) = listener.accept().await.expect("TcpPull, failure accepting connection");
                    let tx2 = tx3.clone();    
                    // Spawn our handler to be run asynchronously.
                    println!("came in guest");
                    tokio::spawn(async move {
                        if let Err(e) = Self::process(socket, tx2).await {
                        }
                    });
            //    }
            })
            /*
            loop {
                // Asynchronously wait for an inbound TcpStream.
                let (stream, addr) = listener.accept().await.expect("TcpPull, failure accepting connection");
                let tx2 = tx3.clone();    
                // Spawn our handler to be run asynchronously.
                println!("came in guest");
                tokio::spawn(async move {
                    if let Err(e) = Self::process(stream, addr, tx2).await {
                    }
                });
            }*/
    };
        let handle= thread::spawn(worker);

        Self {
           // listener,
            handle:handle,
            rx,
            tx: tx.clone()
        }
    }

    pub async fn recv(&mut self) -> Option<Bytes> {

        self.rx.recv().await
        
    }
    






// Process an individual chat client
async fn process(
    socket: UdpSocket,
    //addr: SocketAddr,
    tx: mpsc::Sender<Bytes>
) -> Result<(), Box<dyn Error>> {
    //let mut lines = FramedRead::new(stream, LengthDelimitedCodec::new());
    let mut frames = UdpFramed::new(socket, BytesCodec::new());

    loop {

    match frames.try_next().await {
        // A message was received from the current user, we should
        // broadcast this message to the other users.
        Ok(Some(msg_socket)) => {

            tx.send(msg_socket.0.into()).await;
        }
        // An error occurred.
        Err(e) => {
            println!(
                "an error occurred while processing messages error = {:?}",
                e
            );
        }
        // The stream has been exhausted.
        Ok(None) => {
            println!("pull stream exhausted")
        },
    }
}
    println!("done");
    Ok(())
}


}