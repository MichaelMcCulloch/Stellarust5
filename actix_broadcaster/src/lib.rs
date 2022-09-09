use std::{
    pin::Pin,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll},
    time::Duration,
};

use actix_rt::time::{interval_at, Instant};
use bytes::Bytes;
use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use crtq::{channel, Consumer, Producer};
use futures::Stream;
use serde::Serialize;

pub struct Broadcaster {
    producer: Producer<Sender<Bytes>>,
    consumer: Consumer<Sender<Bytes>>,
}
impl Broadcaster {
    const PING_INTERVAL: u64 = 10;

    pub fn create() -> Self {
        let (producer, consumer) = channel(16, 16);
        let me = Broadcaster { producer, consumer };
        me.spawn_ping();
        me
    }

    pub fn new_client(&self) -> Client {
        let (bytes_sender, bytes_receiver) = unbounded();

        bytes_sender
            .try_send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.producer.produce(bytes_sender).unwrap();

        Client(bytes_receiver)
    }

    pub fn send<S: Serialize>(&self, message: &S) {
        log::info!("sending a message ");
        let msg = Bytes::from(
            [
                "event: message\ndata: ",
                serde_json::to_string(message).unwrap().as_str(),
                "\n\n",
            ]
            .concat(),
        );

        let mut write_back = vec![];

        let p = self.producer.clone();
        let c = self.consumer.clone();

        while let Ok(sender) = c.consume() {
            log::info!("Found a sender");
            match sender.try_send(msg.clone()) {
                Ok(_) => log::info!("Success"),
                Err(e) => log::warn!("Failed to send message"),
            };
            write_back.push(sender);
        }
        while let Some(sender) = write_back.pop() {
            p.produce(sender).unwrap();
        }
    }

    fn spawn_ping(&self) {
        let producer = self.producer.clone();
        let consumer = self.consumer.clone();
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(Self::PING_INTERVAL));

            loop {
                task.tick().await;
                Self::remove_stale_clients(&producer, &consumer);
            }
        });
    }
    fn remove_stale_clients(
        producer: &Producer<Sender<Bytes>>,
        consumer: &Consumer<Sender<Bytes>>,
    ) {
        let producer = producer.clone();
        let consumer = consumer.clone();
        let mut write_back = vec![];

        while let Ok(sender) = consumer.consume() {
            if sender
                .send(Bytes::from("event: ping\ndata: ping\n\n"))
                .is_ok()
            {
                write_back.push(sender);
            } else {
                log::info!("removing sender")
            }
        }
        while let Some(sender) = write_back.pop() {
            producer.produce(sender).unwrap();
        }
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
