#[cfg(test)]
mod tests {

    use std::{sync::Arc, thread, time::Duration};

    use actix_broadcaster::{ActixBroadcaster, Broadcaster, TestClient};
    use bytes::Bytes;
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
