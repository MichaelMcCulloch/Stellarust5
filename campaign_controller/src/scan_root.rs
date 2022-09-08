use std::{
    fs::DirEntry,
    io::Error,
    path::{Path, PathBuf},
};

use directory_watcher::Startup;

pub struct ScanSubdirectoriesOfRootForLatestFile;

impl ScanSubdirectoriesOfRootForLatestFile {
    fn select_directories(entry: Result<DirEntry, Error>) -> Option<DirEntry> {
        match entry {
            Ok(dir_entry) => {
                if dir_entry.path().is_dir() {
                    Some(dir_entry)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn select_files(sub_entry: Result<DirEntry, Error>) -> Option<DirEntry> {
        match sub_entry {
            Ok(file_entry) => {
                if file_entry.path().is_file() {
                    Some(file_entry)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
    fn select_most_recently_modified_file(mut child_items: Vec<DirEntry>) -> DirEntry {
        child_items.sort_by(|a, b| {
            b.metadata()
                .unwrap()
                .modified()
                .unwrap()
                .cmp(&a.metadata().unwrap().modified().unwrap())
        });
        child_items.remove(0)
    }
}

impl Startup for ScanSubdirectoriesOfRootForLatestFile {
    fn startup(&self, root_directory: &Path) -> Vec<PathBuf> {
        std::fs::read_dir(root_directory)
            .unwrap()
            .filter_map(|entry| Self::select_directories(entry))
            .map(|subdirectory| {
                let child_items = std::fs::read_dir(subdirectory.path())
                    .unwrap()
                    .filter_map(|sub_entry| Self::select_files(sub_entry))
                    .collect::<Vec<_>>();

                Self::select_most_recently_modified_file(child_items).path()
            })
            .collect::<Vec<_>>()
    }
}
