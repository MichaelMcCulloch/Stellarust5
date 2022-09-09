use std::{
    pin::Pin,
    sync::mpsc::{channel, Receiver, Sender},
    task::{Context, Poll},
    thread,
    time::Duration,
};

use bytes::Bytes;
use crtq::{channel as queue, Consumer, Producer};
use futures::Stream;
use serde::Serialize;

pub struct Broadcaster {
    producer: Producer<Sender<Bytes>>,
    consumer: Consumer<Sender<Bytes>>,
}
impl Broadcaster {
    const PING_INTERVAL: u64 = 10;

    pub fn create() -> Self {
        let (producer, consumer) = queue(16, 16);
        let me = Broadcaster { producer, consumer };
        me.spawn_ping();
        me
    }

    pub fn new_client(&self) -> Client {
        let (bytes_sender, bytes_receiver) = channel();

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

    fn spawn_ping(&self) {
        let producer = self.producer.clone();
        let consumer = self.consumer.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(Self::PING_INTERVAL));
            futures::executor::block_on(Self::remove_stale_clients(&producer, &consumer));
        });
    }
    async fn remove_stale_clients(
        producer: &Producer<Sender<Bytes>>,
        consumer: &Consumer<Sender<Bytes>>,
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

pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, actix_web::http::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).try_recv() {
            Ok(v) => Poll::Ready(Some(Ok(v))),
            Err(std::sync::mpsc::TryRecvError::Empty) => Poll::Ready(None),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Poll::Pending,
        }
    }
}
