use bytes::Bytes;
use crossbeam::thread::Scope;
use crtq::{channel as queue, Consumer, Producer};
use futures::Stream;
use rayon::prelude::*;
use serde::Serialize;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
pub struct Broadcaster {
    producer: Producer<UnboundedSender<Bytes>>,
    consumer: Consumer<UnboundedSender<Bytes>>,
}
impl Broadcaster {
    const PING_INTERVAL: u64 = 10;

    pub fn create(scope: &Scope<'_>) -> Self {
        let (producer, consumer) = queue(16, 16);
        let me = Broadcaster { producer, consumer };
        me.spawn_ping(scope);
        me
    }

    pub fn new_client(&self) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded_channel();

        bytes_sender
            .send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.producer.produce(bytes_sender).unwrap();

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
        log::info!("sending {} to new client", message_string);
        self.producer.produce(bytes_sender).unwrap();

        Client(bytes_receiver)
    }

    pub fn send<S: Serialize>(&self, message: &S) {
        let message_string = &serde_json::to_string(&message).unwrap();
        let message_bytes =
            Bytes::from(["event: message\ndata: ", message_string, "\n\n"].concat());

        let p = self.producer.clone();
        let c = self.consumer.clone();

        let mut write_back = vec![];

        while let Ok(sender) = c.consume() {
            write_back.push(sender);
        }

        write_back.par_iter().for_each(|sender| {
            sender.send(message_bytes.clone()).unwrap();
        });

        log::info!("sending {} to {} clients", message_string, write_back.len());

        while let Some(sender) = write_back.pop() {
            p.produce(sender).unwrap();
        }
    }

    fn spawn_ping(&self, scope: &Scope<'_>) {
        let producer = self.producer.clone();
        let consumer = self.consumer.clone();
        scope.spawn(move |_| loop {
            std::thread::sleep(Duration::from_secs(Self::PING_INTERVAL));
            futures::executor::block_on(Self::remove_stale_clients(&producer, &consumer));
        });
    }
    async fn remove_stale_clients(
        producer: &Producer<UnboundedSender<Bytes>>,
        consumer: &Consumer<UnboundedSender<Bytes>>,
    ) {
        let mut write_back = vec![];

        while let Ok(sender) = consumer.consume() {
            write_back.push(sender);
        }

        let mut map = write_back
            .par_drain(..)
            .filter_map(|sender| {
                if sender
                    .send(Bytes::from("event: ping\ndata: ping\n\n"))
                    .is_ok()
                {
                    Some(sender)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        log::info!("Retaining {} clients", map.len());
        while let Some(sender) = map.pop() {
            producer.produce(sender).unwrap();
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
