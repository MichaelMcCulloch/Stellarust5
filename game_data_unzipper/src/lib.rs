use std::{fs::File, io::Read, path::Path, vec};

use memmap::Mmap;
use zip::ZipArchive;

pub fn get_zipped_content(path: &Path) -> (String, String) {
    let zip_file = File::open(path).expect(format!("Did not find a file at {:?}", path).as_str());
    let mut zip_archive = ZipArchive::new(zip_file)
        .expect(format!("Expected file at {:?} to be a zip archive", path).as_str());

    let mut meta_bytes: Vec<u8> = vec![];
    let mut gamestate_bytes: Vec<u8> = vec![];
    let mut meta = String::new();
    let mut gamestate = String::new();

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i).unwrap();

        match file.enclosed_name() {
            Some(path) => match path.to_str() {
                Some("meta") => {
                    file.read_to_end(&mut meta_bytes).unwrap();
                    meta = String::from(String::from_utf8_lossy(&meta_bytes))
                }
                Some("gamestate") => {
                    file.read_to_end(&mut gamestate_bytes).unwrap();
                    gamestate = String::from(String::from_utf8_lossy(&gamestate_bytes))
                }
                Some(_) | None => {}
            },
            None => {}
        }
    }
    (meta, gamestate)
}
