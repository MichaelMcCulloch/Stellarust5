use crate::filter;
use crate::scan_root;
use crossbeam::channel::Sender;
use directory_watcher::{DefaultWatcher, DirectoryWatcher, RecursiveMode};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use game_data_info_struct_reader::{GameDataInfoStructReader, ModelDataPoint};
use notify::RecommendedWatcher;
use scan_root::ScanAllFoldersAndFiles;
use std::path::Path;

pub(crate) fn get_directory_watcher(
    info_struct_sender: Sender<ModelDataPoint>,
    game_directory: &Path,
) -> RecommendedWatcher {
    DefaultWatcher::create_directory_watcher_and_scan_root(
        CloseWriteFilter,
        EndsWithSavFilter,
        GameDataInfoStructReader,
        move |message: ModelDataPoint| -> () {
            log::trace!("Discovered {:?} -- {}", message.date, message.campaign_name);

            info_struct_sender.send(message).unwrap();
        },
        ScanAllFoldersAndFiles,
        &game_directory,
        RecursiveMode::Recursive,
    )
}
