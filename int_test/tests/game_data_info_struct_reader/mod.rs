#[cfg(test)]
mod tests {
    use game_data_info_struct_reader::{GameDataInfoStructReader, ModelDataPoint};
    use std::path::PathBuf;
    use stellarust::PROD_TEST_DATA_ROOT;
    use trait_file_reader::FileReader;

    const INT_TEST_ROOT: &str = &"/home/michael/Dev/Stellarust/stellarust5/int_test/";
    

    #[test]
    fn test_name() {
        let mut reference = PathBuf::from(INT_TEST_ROOT);
        reference.push("resource/unitednationsofearth/2200.01.01.serialize");
        let mut savegame = PathBuf::from(PROD_TEST_DATA_ROOT);
        savegame.push(&"unitednationsofearth/2200.01.01.sav");
        let model_data = GameDataInfoStructReader.read_file(savegame.as_path());

        let reference_string = std::fs::read_to_string(reference).unwrap();
        let reference_model_data: ModelDataPoint =
            serde_json::from_str(reference_string.as_str()).unwrap();

        assert_eq!(reference_model_data, model_data);
    }
}
