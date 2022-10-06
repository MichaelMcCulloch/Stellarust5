pub const START_OF_GAME_YEAR: i32 = 2200;
pub const START_OF_GAME_MONTH: u32 = 1;
pub const START_OF_GAME_DATE: u32 = 1;

#[cfg(target_os = "linux")]
pub const PROD_TEST_EMPTY_FOLDER: &str =
    &"/home/michael/Dev/Stellarust/stellarust5/production_data/empty_folder";
#[cfg(target_os = "windows")]
pub const PROD_TEST_EMPTY_FOLDER: &str =
    &"C:\\Users\\micha\\Dev\\Stellarust5\\production_data\\empty_folder";

