use std::{fs::File, io::Read, path::Path};

use zip::ZipArchive;

pub fn get_zipped_content(path: &Path) -> (String, String) {
    let zip_file = File::open(path).expect(format!("Did not find a file at {:?}", path).as_str());
    let mut zip_archive = ZipArchive::new(zip_file)
        .expect(format!("Expected file at {:?} to be a zip archive", path).as_str());

    let mut meta = String::new();
    let mut gamestate = String::new();

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i).unwrap();

        match file.enclosed_name() {
            Some(path) => match path.to_str() {
                Some("meta") => {
                    file.read_to_string(&mut meta).unwrap();
                }
                Some("gamestate") => {
                    file.read_to_string(&mut gamestate).unwrap();
                }
                Some(_) | None => {}
            },
            None => {}
        }
    }
    drop(zip_archive);
    (meta, gamestate)
}
