use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, FramedRead, LengthDelimitedCodec};

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
pub struct TcpPull {
    //listener: TcpListener,
    rx: mpsc::Receiver<Bytes>,
    tx: mpsc::Sender<Bytes>,
    //handle: thread::JoinHandle<(tokio::task::JoinHandle<()>)>
}

impl TcpPull {
    pub async fn connect(addr: &str) -> Self {
        // Note that this is the Tokio TcpListener, which is fully async.
        let stream = TcpStream::connect(&addr).await.expect("TcpPull, failed to connect");
        let (tx, rx) = mpsc::channel(10000);
        let tx3 = tx.clone();
        let rt = tokio::runtime::Handle::current();
       
        
        let tx2 = tx3.clone();    
        // Spawn our handler to be run asynchronously.
        println!("came in guest");
        tokio::spawn(async move {
            if let Err(e) = Self::process(stream, tx2).await {
            }
                    });
             

                
      

        Self {
           // listener,
            //handle:handle,
            rx,
            tx: tx.clone()
        }
    }

    pub async fn recv(&mut self) -> Option<Bytes> {
        
        self.rx.recv().await
        
    }
    






// Process an individual chat client
async fn process(
    stream: TcpStream,
    //addr: SocketAddr,
    tx: mpsc::Sender<Bytes>
) -> Result<(), Box<dyn Error>> {
    let mut lines = FramedRead::new(stream, LengthDelimitedCodec::new());

    loop {
    match lines.next().await {
        // A message was received from the current user, we should
        // broadcast this message to the other users.
        Some(Ok(msg)) => {

            tx.send(msg.into()).await;
        }
        // An error occurred.
        Some(Err(e)) => {
            println!(
                "an error occurred while processing messages error = {:?}",
                e
            );
        }
        // The stream has been exhausted.
        None => {
            println!("pull stream exhausted")
        },
    }
}
    println!("done");
    Ok(())
}


}