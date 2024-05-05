use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // send welcome message to the client
    ws_stream.send(Message::text("Welcome to Chat App! Type your message.".to_string())).await?;

    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // receive message from client
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(text) = msg.as_text() {
                    println!("from client {:?}: {:?}", addr, text);
                    bcast_tx.send(format!("{addr} : {text}"))?;
                }
            }
            // receive broadcast message
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
            // if client's stream ends, return
            else => return Ok(()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from Bryan Raihan 'Ilman's computer {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}