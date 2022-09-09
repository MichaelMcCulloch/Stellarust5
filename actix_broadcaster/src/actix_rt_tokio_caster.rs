use std::{
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::Duration,
};

use actix_rt::time::{interval_at, Instant};
use bytes::Bytes;
use crossbeam::thread::Scope;
use crtq::{channel as queue, Consumer, Producer};
use futures::Stream;
use serde::Serialize;
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

    pub fn send<S: Serialize>(&self, message: &S) {
        log::info!("sending a message ");
        let msg = Bytes::from(
            [
                "event: message\ndata: ",
                "duck", //,
                "\n\n",
            ]
            .concat(),
        );

        let mut write_back = vec![];

        let p = self.producer.clone();
        let c = self.consumer.clone();

        while let Ok(sender) = c.consume() {
            log::info!("Found a sender");
            match sender.send(msg.clone()) {
                Ok(_) => log::info!("Success"),
                Err(e) => log::warn!("Failed to send message"),
            };
            write_back.push(sender);
        }
        while let Some(sender) = write_back.pop() {
            p.produce(sender).unwrap();
        }
    }

    fn spawn_ping(&self, scope: &Scope<'_>) {
        let producer = self.producer.clone();
        let consumer = self.consumer.clone();
        scope.spawn(move |s| {
            let mut task = interval_at(Instant::now(), Duration::from_secs(10));
            futures::executor::block_on(task.tick());
            futures::executor::block_on(Self::remove_stale_clients(&producer, &consumer));
        });
    }
    async fn remove_stale_clients(
        producer: &Producer<UnboundedSender<Bytes>>,
        consumer: &Consumer<UnboundedSender<Bytes>>,
    ) {
        let producer = producer.clone();
        let consumer = consumer.clone();
        let mut write_back = vec![];

        let mut count = 0;
        while let Ok(sender) = consumer.consume() {
            if sender
                .send(Bytes::from("event: ping\ndata: ping\n\n"))
                .is_ok()
            {
                write_back.push(sender);
            } else {
                count += 1
            }
        }
        log::info!("Removed {:?} stale clients", count);
        while let Some(sender) = write_back.pop() {
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
            Poll::Ready(None) => Poll::Pending,
            Poll::Pending => Poll::Pending,
        }
    }
}
