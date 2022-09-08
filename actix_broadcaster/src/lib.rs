use std::{
    pin::Pin,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll},
    time::Duration,
};

use actix_rt::time::{interval_at, Instant};
use bytes::Bytes;
use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::Stream;
use serde::Serialize;

pub struct Broadcaster {
    clients: Arc<RwLock<Vec<Sender<Bytes>>>>,
}
impl Broadcaster {
    const PING_INTERVAL: u64 = 10;

    pub fn create() -> Self {
        let clients = Arc::new(RwLock::new(Vec::new()));
        Broadcaster::spawn_ping(clients.clone());
        Broadcaster { clients: clients }
    }

    pub fn new_client(&self) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded();

        bytes_sender
            .try_send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        {
            let mut clients_lock = self.clients.write().unwrap();
            (*clients_lock).push(bytes_sender);
        }
        Client(bytes_receiver)
    }

    pub fn send<S: Serialize>(&self, message: &S) {
        let msg = Bytes::from(
            [
                "event: message\ndata: ",
                serde_json::to_string(message).unwrap().as_str(),
                "\n\n",
            ]
            .concat(),
        );

        for client in self.clients.read().unwrap().iter() {
            client.try_send(msg.clone()).unwrap_or(());
        }
    }

    fn spawn_ping(clients: Arc<RwLock<Vec<Sender<Bytes>>>>) {
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(Self::PING_INTERVAL));
            loop {
                task.tick().await;
                Self::remove_stale_clients(&clients);
            }
        });
    }
    fn remove_stale_clients(clients: &Arc<RwLock<Vec<Sender<Bytes>>>>) {
        clients.write().unwrap().retain(|client| {
            client
                .try_send(Bytes::from("event: ping\ndata: ping\n\n"))
                .is_ok()
        });
    }
}

pub struct Client(Receiver<Bytes>);
impl Stream for Client {
    type Item = Result<Bytes, actix_web::http::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).try_recv() {
            Ok(bytes) => Poll::Ready(Some(Ok(bytes))),
            Err(TryRecvError::Disconnected) => Poll::Pending,
            Err(TryRecvError::Empty) => Poll::Ready(None),
        }
    }
}
