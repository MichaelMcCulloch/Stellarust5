#[cfg(test)]
mod tests {
    use std::{path::PathBuf, thread::sleep, time::Duration};

    use campaign_controller::CampaignController;
    use crossbeam::thread;

    use super::*;
    #[actix_rt::test]
    async fn test_name() {
        let x = CampaignController::create(&PathBuf::from(
            "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/",
        ));
        loop {
            println!("this is the main thread");
            sleep(Duration::from_secs(1));
        }
    }
}
