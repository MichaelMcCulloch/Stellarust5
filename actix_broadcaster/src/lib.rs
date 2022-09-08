use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};

use actix_rt::time::{interval_at, Instant};
use bytes::Bytes;
use crossbeam::{
    atomic::AtomicCell,
    channel::{unbounded, Receiver, Sender, TryRecvError},
};
use futures::Stream;
use serde::Serialize;

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}
impl Broadcaster {
    const PING_INTERVAL: u64 = 10;
    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    pub fn create() -> Arc<Mutex<Self>> {
        let me = Arc::new(Mutex::new(Broadcaster::new()));
        Broadcaster::spawn_ping(me.clone());
        me
    }

    pub fn new_client(&mut self) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded();

        bytes_sender
            .try_send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.clients.push(bytes_sender);
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

        for client in self.clients.iter() {
            client.try_send(msg.clone()).unwrap_or(());
        }
    }

    fn spawn_ping(me: Arc<Mutex<Self>>) {
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(Self::PING_INTERVAL));
            loop {
                task.tick().await;
                me.lock().unwrap().remove_stale_clients()
            }
        });
    }
    fn remove_stale_clients(&mut self) {
        self.clients = self
            .clients
            .iter()
            .filter_map(|client| {
                match client.try_send(Bytes::from("event: ping\ndata: ping\n\n")) {
                    Ok(_) => Some(client.clone()),
                    Err(_) => None,
                }
            })
            .collect();
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
