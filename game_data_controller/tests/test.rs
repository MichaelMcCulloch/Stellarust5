#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crossbeam::thread;
    use game_data_controller::GameModelController;

    use super::*;
    #[test]
    fn test_name() {
        thread::scope(|s| {
            let controller = GameModelController::create(
                &PathBuf::from(
                    "/home/michael/Dev/Stellarust/stellarust5/production_data/3.4.5.95132",
                ),
                s,
            );
        })
        .unwrap();
    }
}
