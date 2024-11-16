use tokio::sync::mpsc::UnboundedReceiver;
use warp::ws::Message;
use futures::SinkExt;

use crate::models::ws::WsSink;

pub async fn start_forwarding(mut receiver: UnboundedReceiver<Message>, mut sink: WsSink) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if let Err(err) = sink.send(message).await {
                eprintln!("Could not forward message. {err:?}");
                break;
            }
        }
    })
}
