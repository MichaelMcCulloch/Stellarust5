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
    self_destruct: tokio::sync::mpsc::Sender<()>,
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
    fn create(self_destruct: tokio::sync::mpsc::Sender<()>) -> Self;
    fn new_client(&self) -> Client;
    fn new_client_with_message<S: Serialize>(&self, message: &S) -> Client;
    fn send<S: Serialize>(&self, message: &S) -> usize;
}

impl Broadcaster for ActixBroadcaster {
    fn create(self_destruct: tokio::sync::mpsc::Sender<()>) -> Self {
        let me = ActixBroadcaster {
            clients: Arc::new(RwLock::new(vec![])),
            self_destruct,
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
        let mut write_guard = self.clients.write().unwrap();
        let mut clients = std::mem::take(&mut *write_guard);
        let message_string = &serde_json::to_string(&message).unwrap();
        let message_bytes =
            Bytes::from(["event: message\ndata: ", message_string, "\n\n"].concat());

        let (clients, count) = clients
            .par_drain(..)
            .fold(
                || (Vec::new(), 0),
                |(mut accumulator, count), sender| match sender.send(message_bytes.clone()) {
                    Ok(_) => {
                        accumulator.push(sender);
                        (accumulator, count + 1)
                    }
                    Err(_) => (accumulator, count),
                },
            )
            .reduce(
                || (Vec::new(), 0),
                |(mut va, ca), (vb, cb)| {
                    va.extend(vb.into_iter());
                    (va, ca + cb)
                },
            );
        *write_guard = clients;
        count
    }
}

impl ActixBroadcaster {
    fn spawn_ping(&self) {
        let clients = self.clients.clone();
        let self_destruct = self.self_destruct.clone();
        actix_web::rt::spawn(async move {
            let mut interval = actix_web::rt::time::interval(Self::PING_INTERVAL);
            loop {
                interval.tick().await;
                if Self::remove_stale_clients(&clients, &self_destruct).await {
                    break;
                };
            }
        });
    }
    async fn remove_stale_clients(
        clients: &Arc<RwLock<Vec<UnboundedSender<Bytes>>>>,
        self_destruct: &tokio::sync::mpsc::Sender<()>,
    ) -> bool {
        let var_name = &"event: ping\ndata: ping\n\n";

        let mut write_guard = clients.write().unwrap();
        let mut clients = std::mem::take(&mut *write_guard);

        let (clients, count) = clients
            .par_drain(..)
            .fold(
                || (Vec::new(), 0),
                |(mut accumulator, count), sender| match sender.send(Bytes::from(*var_name)) {
                    Ok(_) => {
                        accumulator.push(sender);
                        (accumulator, count + 1)
                    }
                    Err(_) => (accumulator, count),
                },
            )
            .reduce(
                || (Vec::new(), 0),
                |(mut va, ca), (vb, cb)| {
                    va.extend(vb.into_iter());
                    (va, ca + cb)
                },
            );
        *write_guard = clients;
        log::trace!("Ping: Retaining {} clients", count);
        if count == 0 {
            match self_destruct.send(()).await {
                _ => {}
            };
            true
        } else {
            false
        }
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
        let (t, _r) = tokio::sync::mpsc::channel(1);
        let b = ActixBroadcaster::create(t);

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
        let (t, _r) = tokio::sync::mpsc::channel(1);
        let b = ActixBroadcaster::create(t);

        let mut x = b.new_client();
        match x.recv().await {
            Some(x) => assert_eq!(x, Bytes::from("event: connected\ndata: connected\n\n")),
            None => assert!(false, "Connection Message Not Received"),
        }
    }
    #[actix_rt::test]
    async fn broadcaster_existing_client_send() {
        let (t, _r) = tokio::sync::mpsc::channel(1);
        let b = ActixBroadcaster::create(t);

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
        let (t, _r) = tokio::sync::mpsc::channel(1);
        let b = ActixBroadcaster::create(t);

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
