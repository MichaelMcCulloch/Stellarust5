use crate::filter::EndsWithSavFilter;
use directory_watcher::{PathFilter, Startup};
use std::{
    fs::DirEntry,
    io::Error,
    path::{Path, PathBuf},
};

pub struct ScanAllFoldersAndFiles;

impl ScanAllFoldersAndFiles {
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

    fn select_files_ending_in_sav(sub_entry: Result<DirEntry, Error>) -> Option<DirEntry> {
        match sub_entry {
            Ok(file_entry) => {
                if EndsWithSavFilter.filter_path(&file_entry.path()) {
                    Some(file_entry)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
impl Startup for ScanAllFoldersAndFiles {
    fn startup(&self, root_directory: &Path) -> Vec<PathBuf> {
        let collect = std::fs::read_dir(root_directory)
            .unwrap()
            .filter_map(|entry| Self::select_directories(entry))
            .flat_map(|subdirectory| {
                std::fs::read_dir(subdirectory.path())
                    .unwrap()
                    .filter_map(
                        |sub_entry| match Self::select_files_ending_in_sav(sub_entry) {
                            Some(dir) => Some(dir.path()),
                            None => None,
                        },
                    )
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        collect
    }
}
