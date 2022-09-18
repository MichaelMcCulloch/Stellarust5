use actix_rt::signal::ctrl_c;
use actix_server::run_app;
use actix_web::rt;
use crossbeam::{channel::unbounded, thread};
fn main() -> Result<(), Box<(dyn std::any::Any + Send + 'static)>> {
    thread::scope(|scope| {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();

        let (sender, receiver) = unbounded();

        scope.spawn(|scope| -> Result<_, std::io::Error> {
            let server_future = run_app(sender, scope);
            rt::System::new().block_on(server_future)
        });

        let _server_handle = receiver.recv().unwrap();
    })
    .unwrap();
    Ok(())
}
