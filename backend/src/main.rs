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
                    actix_broadcaster={},
                    actix_server={},
                    directory_watcher={},
                    game_data_controller={},
                    game_data_info_struct={},
                    game_data_info_struct_reader={},
                    game_data_unzipper={},
                    model_info_struct={}  
                "###,
                "info", "info", "trace", "info", "trace", "trace", "trace", "trace"
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
