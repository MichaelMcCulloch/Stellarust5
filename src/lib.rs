#[cfg(target_os= "linux")]
pub const PROD_TEST_DATA_ROOT: &str =
    &"/home/michael/Dev/Stellarust/stellarust5/production_data/3.4.5.95132";
#[cfg(target_os = "windows")]
pub const PROD_TEST_DATA_ROOT: &str =
    &"C:\\Users\\micha\\Dev\\Stellarust5\\production_data\\3.4.5.95132";

#[cfg(target_os= "linux")]
pub const PROD_TEST_EMPTY_FOLDER: &str =
    &"/home/michael/Dev/Stellarust/stellarust5/production_data/empty_folder";
#[cfg(target_os = "windows")]
    pub const PROD_TEST_EMPTY_FOLDER: &str =
    &"C:\\Users\\micha\\Dev\\Stellarust5\\production_data\\empty_folder";
