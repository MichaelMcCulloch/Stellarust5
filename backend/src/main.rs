use std::path::Path;

use actix_server::{run_actix_server, run_actix_server_https};
use actix_web::rt;
use anyhow::Result;
use crossbeam::thread;
use stellarust::{PROD_TEST_DATA_ROOT, PROD_TEST_EMPTY_FOLDER, STELLARIS_SAVE_ROOT};
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
        let system_runner = rt::System::new();
        let game_data_root = &Path::new(STELLARIS_SAVE_ROOT);
        let server = match (
            std::env::var("STELLARUST_KEY"),
            std::env::var("STELLARUST_CERT"),
        ) {
            (Ok(key), Ok(cert)) => {
                let server_future = run_actix_server_https(
                    scope,
                    game_data_root,
                    Path::new(&key),
                    Path::new(&cert),
                );
                system_runner.block_on(server_future).unwrap()
            }
            _ => system_runner
                .block_on(run_actix_server(scope, game_data_root))
                .unwrap(),
        };

        system_runner.block_on(server).unwrap()
    })
    .unwrap();
    Ok(())
}
