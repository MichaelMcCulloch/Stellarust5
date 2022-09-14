use actix_rt::time::{interval_at, Instant};
use bytes::Bytes;

use futures::Stream;
use rayon::prelude::*;
use serde::Serialize;
use std::{
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub struct Broadcaster {
    clients: Arc<RwLock<Vec<UnboundedSender<Bytes>>>>,
}
impl Broadcaster {
    const PING_INTERVAL: u64 = 10;

    pub fn create() -> Self {
        let me = Broadcaster {
            clients: Arc::new(RwLock::new(vec![])),
        };
        me.spawn_ping();
        me
    }

    pub fn new_client(&self) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded_channel();

        bytes_sender
            .send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.clients.write().unwrap().push(bytes_sender);

        Client(bytes_receiver)
    }
    pub fn new_client_with_message<S: Serialize>(&self, message: &S) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded_channel();
        let message_string = &serde_json::to_string(&message).unwrap();
        bytes_sender
            .send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();
        bytes_sender
            .send(Bytes::from(
                ["event: message\ndata: ", message_string, "\n\n"].concat(),
            ))
            .unwrap();
        log::info!("sending message to new client");
        self.clients.write().unwrap().push(bytes_sender);

        Client(bytes_receiver)
    }

    pub fn send<S: Serialize>(&self, message: &S) -> bool {
        let guard = self.clients.read().unwrap();
        if guard.is_empty() {
            false
        } else {
            let message_string = &serde_json::to_string(&message).unwrap();
            let message_bytes =
                Bytes::from(["event: message\ndata: ", message_string, "\n\n"].concat());

            guard
                .par_iter()
                .for_each(|sender| sender.send(message_bytes.clone()).unwrap());
            true
        }
    }

    fn spawn_ping(&self) {
        let clients = self.clients.clone();
        actix_web::rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(Self::PING_INTERVAL));
            loop {
                task.tick().await;
                Self::remove_stale_clients(&clients).await;
            }
        });
    }
    async fn remove_stale_clients(clients: &Arc<RwLock<Vec<UnboundedSender<Bytes>>>>) {
        clients.write().unwrap().retain(|sender| {
            if let Ok(()) = sender.send(Bytes::from("event: ping\ndata: ping\n\n")) {
                true
            } else {
                false
            }
        });
    }
}
pub struct Client(UnboundedReceiver<Bytes>);
impl Stream for Client {
    type Item = Result<Bytes, actix_web::http::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
