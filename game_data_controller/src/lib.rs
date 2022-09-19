use std::{hash::BuildHasherDefault, path::Path, sync::Arc};

use actix_broadcaster::{ActixBroadcaster, Broadcaster, Client};
use controller::Controller;
use crossbeam::{
    channel::{unbounded, Receiver, Sender},
    thread::Scope,
};
use dashmap::{mapref::entry::Entry, DashMap};
use directory_watcher::{DefaultWatcher, DirectoryWatcher, RecursiveMode};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use fxhash::{FxBuildHasher, FxHasher};
use game_data_info_struct_reader::{GameDataInfoStructReader, ModelDataPoint};
use model_info_struct::{
    enums::{ModelEnum, ModelSpecEnum},
    Model,
};
use notify::RecommendedWatcher;
use scan_root::ScanAllFoldersAndFiles;
mod filter;
mod scan_root;

pub mod controller;
