use actix_server::run_actix_server;
use actix_web::rt;
use anyhow::Result;
use crossbeam::thread;
fn main() -> Result<()> {
    thread::scope(|scope| {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();

        let server_future = run_actix_server(scope);
        let system_runner = rt::System::new();
        let server = system_runner.block_on(server_future).unwrap();
        system_runner.block_on(server).unwrap()
    })
    .unwrap();
    Ok(())
}
