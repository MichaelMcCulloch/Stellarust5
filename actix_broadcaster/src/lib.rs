#[allow(unused_imports)]
use async_trait::async_trait;
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
pub struct ActixBroadcaster {
    clients: Arc<RwLock<Vec<UnboundedSender<Bytes>>>>,
}

pub trait Ping {
    const PING_INTERVAL: Duration;
}

#[cfg(not(test))]
impl Ping for ActixBroadcaster {
    const PING_INTERVAL: Duration = Duration::from_secs(10);
}
#[cfg(test)]
impl Ping for ActixBroadcaster {
    const PING_INTERVAL: Duration = Duration::from_millis(10);
}

pub trait Broadcaster {
    fn create() -> Self;
    fn new_client(&self) -> Client;
    fn new_client_with_message<S: Serialize>(&self, message: &S) -> Client;
    fn send<S: Serialize>(&self, message: &S) -> usize;
}

impl Broadcaster for ActixBroadcaster {
    fn create() -> Self {
        let me = ActixBroadcaster {
            clients: Arc::new(RwLock::new(vec![])),
        };
        me.spawn_ping();
        me
    }

    fn new_client(&self) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded_channel();

        bytes_sender
            .send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.clients.write().unwrap().push(bytes_sender);
        Client(bytes_receiver)
    }
    fn new_client_with_message<S: Serialize>(&self, message: &S) -> Client {
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
        self.clients.write().unwrap().push(bytes_sender);

        Client(bytes_receiver)
    }

    fn send<S: Serialize>(&self, message: &S) -> usize {
        let guard = self.clients.read().unwrap();
        if guard.is_empty() {
            0
        } else {
            let message_string = &serde_json::to_string(&message).unwrap();
            let message_bytes =
                Bytes::from(["event: message\ndata: ", message_string, "\n\n"].concat());

            guard
                .par_iter()
                .for_each(|sender| sender.send(message_bytes.clone()).unwrap());
            guard.len()
        }
    }
}

impl ActixBroadcaster {
    fn spawn_ping(&self) {
        let clients = self.clients.clone();
        actix_web::rt::spawn(async move {
            let mut interval = actix_web::rt::time::interval(Self::PING_INTERVAL);
            loop {
                interval.tick().await;
                Self::remove_stale_clients(&clients);
            }
        });
    }
    fn remove_stale_clients(clients: &Arc<RwLock<Vec<UnboundedSender<Bytes>>>>) {
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

#[cfg(test)]
#[async_trait(?Send)]
pub trait TestClient {
    async fn recv(&mut self) -> Option<Bytes>;
}

#[cfg(test)]
#[async_trait(?Send)]
impl TestClient for Client {
    async fn recv(&mut self) -> Option<Bytes> {
        self.0.recv().await
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::{ActixBroadcaster, Broadcaster, TestClient};

    #[actix_rt::test]
    async fn broadcaster_new_client_with_message_x_receives_connection_message_then_message_x() {
        let b = ActixBroadcaster::create();

        let mut x = b.new_client_with_message(&"here is a message");
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: connected\ndata: connected\n\n")),
            None => assert!(false, "Connection Message Not Received"),
        }
        match x.recv().await {
            Some(x) => assert_eq!(
                x,
                Bytes::from("event: message\ndata: \"here is a message\"\n\n")
            ),
            None => assert!(false, "Startup Message Not Received"),
        }
    }

    #[actix_rt::test]
    async fn broadcaster_new_client_receives_connection_message() {
        let b = ActixBroadcaster::create();

        let mut x = b.new_client();
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: connected\ndata: connected\n\n")),
            None => assert!(false, "Connection Message Not Received"),
        }
    }
    #[actix_rt::test]
    async fn broadcaster_existing_client_send() {
        let b = ActixBroadcaster::create();

        let mut x = b.new_client();
        b.send(&"Can you hear me? 1 2 3");
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: connected\ndata: connected\n\n")),
            None => assert!(false, "Connection Message Not Received"),
        }
        match x.recv().await {
            Some(x) => assert_eq!(
                x,
                Bytes::from("event: message\ndata: \"Can you hear me? 1 2 3\"\n\n")
            ),
            None => assert!(false, "Broadcast Message Not Received"),
        }
    }

    #[actix_rt::test]
    async fn broadcaster_ping_client_receives_pings_as_long_as_its_polling() {
        let b = ActixBroadcaster::create();

        let mut x = b.new_client();
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: connected\ndata: connected\n\n")),
            None => assert!(false, "Connection Message Not Received"),
        }

        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: ping\ndata: ping\n\n")),
            None => assert!(false, "Ping Message 1 Not Received"),
        }
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: ping\ndata: ping\n\n")),
            None => assert!(false, "Ping Message 2 Not Received"),
        }
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: ping\ndata: ping\n\n")),
            None => assert!(false, "Ping Message 3 Not Received"),
        }
    }
}
