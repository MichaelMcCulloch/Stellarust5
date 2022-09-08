mod filter;
mod scan_root;

use std::{
    collections::HashMap,
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use actix_broadcaster::{Broadcaster, Client};
use campaign_info_struct::CampaignInfoStruct;
use crossbeam::{
    atomic::AtomicCell,
    channel::{unbounded, Receiver},
    thread::{self, Scope},
};
use directory_watcher::{
    create_directory_watcher_and_scan_root, RecommendedWatcher, RecursiveMode,
};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use reader_campaign_info_struct::CampaignInfoStructReader;
use scan_root::ScanSubdirectoriesOfRootForLatestFile;

pub struct CampaignController {
    campaign_broadcaster: Arc<Mutex<Broadcaster>>,
    campaign_list: Arc<Mutex<HashMap<String, CampaignInfoStruct>>>,
    watcher: RecommendedWatcher,
}

impl CampaignController {
    pub fn create<P: AsRef<Path>>(game_directory: &P, thread_scope: &Scope) -> Self {
        let (info_struct_sender, info_struct_receiver) = unbounded();

        let watcher = create_directory_watcher_and_scan_root(
            CloseWriteFilter,
            EndsWithSavFilter,
            CampaignInfoStructReader,
            move |message| -> () {
                info_struct_sender.send(message).unwrap();
            },
            ScanSubdirectoriesOfRootForLatestFile,
            &game_directory,
            RecursiveMode::Recursive,
        );
        let campaign_list = Arc::new(Mutex::new(HashMap::new()));
        let campaign_broadcaster = Broadcaster::create();
        let campaign_controller = Self {
            campaign_broadcaster: campaign_broadcaster.clone(),
            campaign_list: campaign_list.clone(),
            watcher,
        };

        thread_scope.spawn(move |s| loop {
            match info_struct_receiver.recv() {
                Ok(message) => {
                    CampaignController::reconcile(message, campaign_list.clone());
                    CampaignController::broadcast(
                        campaign_list.clone(),
                        campaign_broadcaster.clone(),
                    )
                }
                Err(_) => {}
            };
        });

        campaign_controller
    }

    pub fn get_client(&self) -> Client {
        self.campaign_broadcaster.lock().unwrap().new_client()
    }

    fn reconcile(
        message: CampaignInfoStruct,
        campaign_list: Arc<Mutex<HashMap<String, CampaignInfoStruct>>>,
    ) {
        campaign_list
            .lock()
            .unwrap()
            .insert(message.campaign_name.clone(), message);
    }

    fn broadcast(
        campaign_list: Arc<Mutex<HashMap<String, CampaignInfoStruct>>>,
        broadcaster: Arc<Mutex<Broadcaster>>,
    ) {
        let mutex_guard = campaign_list.lock().unwrap();
        let message = mutex_guard.deref().clone();
        let broadcaster = broadcaster.lock().unwrap();
        broadcaster.send(message)
    }
}
