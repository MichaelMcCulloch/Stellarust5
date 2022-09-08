#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use campaign_controller::CampaignController;
    use crossbeam::thread;

    use super::*;
    #[actix_rt::test]
    async fn test_name() {
        thread::scope(|s| {
            let x = CampaignController::create(
                &"/home/michael/Dev/Stellarust/stellarust5/int_test/scratch",
                s,
            );
            loop {
                println!("this is the main thread");
                sleep(Duration::from_secs(1));
            }
        })
        .unwrap();
    }
}
