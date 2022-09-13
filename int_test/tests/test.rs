#[cfg(test)]
mod tests {
    use std::{path::PathBuf, thread::sleep, time::Duration};

    use campaign_controller::CampaignController;
    use crossbeam::thread;
    use game_data_controller::GameModelController;

    use super::*;
    #[actix_rt::test]
    async fn test_name() {}
}
