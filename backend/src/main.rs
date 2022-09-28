use actix_server::run_actix_server;
use actix_web::rt;
use anyhow::Result;
use crossbeam::thread;
fn main() -> Result<()> {
    thread::scope(|scope| {
        std::env::set_var(
            "RUST_LOG",
            format!(
                r###"
                    actix_broadcaster={broadcaster_level},
                    actix_server={server_level},
                    directory_watcher={watcher_level},
                    game_data_controller={controller_level},
                    game_data_info_struct_reader={reader_level},
                "###,
                broadcaster_level = "info",
                server_level = "info",
                watcher_level = "trace",
                controller_level = "info",
                reader_level = "trace",
            ),
        );
        env_logger::init();

        let server_future = run_actix_server(scope);
        let system_runner = rt::System::new();
        let server = system_runner.block_on(server_future).unwrap();
        system_runner.block_on(server).unwrap()
    })
    .unwrap();
    Ok(())
}
