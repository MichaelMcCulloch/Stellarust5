use actix_server::run_actix_server;
use actix_web::rt;
use anyhow::Result;
use crossbeam::thread;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const PATHS: [&str; 2] = [&"Documents\\Paradox Interactive\\Stellaris\\save games", &"OneDrive\\Documents\\Paradox Interactive\\Stellaris\\save games"];

#[cfg(target_os = "linux")]
const PATHS: [&str; 1] = [&".local/share/Paradox Interactive/Stellaris/save games"];

#[cfg(target_os = "windows")]
const HOME: &str = "USERPROFILE";
#[cfg(target_os = "linux")]
const HOME: &str = "HOME";

fn main() -> Result<()> {
    std::env::set_var(
        "RUST_LOG",
        format!(
            r###"
                stellarust={stellarust_level},
                actix_broadcaster={broadcaster_level},
                actix_server={server_level},
                directory_watcher={watcher_level},
                game_data_controller={controller_level},
                game_data_info_struct_reader={reader_level},
            "###,
            stellarust_level = "info",
            broadcaster_level = "info",
            server_level = "info",
            watcher_level = "info",
            controller_level = "info",
            reader_level = "info",
        ),
    );
    env_logger::init();
    let env_home =
        PathBuf::from(std::env::var(HOME).expect(format!("${} is not defined!", HOME).as_str()));
    let mut save_dir = Option::None;

    let mut tried = vec![];
    for p in PATHS {
        let dir = {
            let mut home = env_home.clone();
            home.push(p);
            home
        };
        if dir.is_dir() {
            log::info!("Using save directory found at {}", dir.display());
            save_dir = Some(dir);
        } else {
            tried.push(dir)
        }
    }
    if let Some(game_data_root) = save_dir {
        thread::scope(|scope| {
            let system_runner = rt::System::new();

            let tls_key = std::env::var("STELLARUST_KEY")
                .map(|s| PathBuf::from(s))
                .ok();
            let tls_cert = std::env::var("STELLARUST_CERT")
                .map(|s| PathBuf::from(s))
                .ok();
            let server_future = run_actix_server(scope, &game_data_root, &tls_key, &tls_cert);
            let server = system_runner.block_on(server_future).unwrap();

            system_runner.block_on(server).unwrap()
        })
        .unwrap();
        Ok(())
    } else {
        Err(anyhow::Error::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Save Data directory not found! Tried {:?}", tried),
        )))
    }
}
 