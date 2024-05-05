use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // receive message from server
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(text) = msg.as_text() {
                    println!("Bryan Raihan 'Ilman's computer - from server: {}", text);
                }
            }
            // send message from stdin
            res = stdin.next_line() => {
                match res {
                    Ok(Some(line)) => ws_stream.send(Message::text(line)).await?,
                    _ => return Ok(()), // Exit loop on None or Error
                }
            }
        }
    }}