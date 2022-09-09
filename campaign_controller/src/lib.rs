mod filter;
mod scan_root;

use std::{
    collections::HashMap,
    ops::Deref,
    path::Path,
    sync::{Arc, RwLock},
    thread,
};

use actix_broadcaster::{Broadcaster, Client};
use actix_rt::ArbiterHandle;
use campaign_info_struct::CampaignInfoStruct;
use crossbeam::{
    channel::{unbounded, Receiver},
    thread::Scope,
};
use directory_watcher::{
    create_directory_watcher_and_scan_root, RecommendedWatcher, RecursiveMode,
};
use filter::{CloseWriteFilter, EndsWithSavFilter};
use reader_campaign_info_struct::CampaignInfoStructReader;
use scan_root::ScanSubdirectoriesOfRootForLatestFile;

pub struct CampaignController {
    campaign_broadcaster: Arc<Broadcaster>,
    campaign_list: Arc<RwLock<HashMap<String, CampaignInfoStruct>>>,
    _watcher: RecommendedWatcher,
}

impl CampaignController {
    pub fn create(game_directory: &Path, scope: &Scope<'_>) -> Self {
        let (info_struct_sender, info_struct_receiver) = unbounded();

        let watcher = create_directory_watcher_and_scan_root(
            CloseWriteFilter,
            EndsWithSavFilter,
            CampaignInfoStructReader,
            move |message| -> () {
                info_struct_sender.clone().send(message).unwrap();
            },
            ScanSubdirectoriesOfRootForLatestFile,
            &game_directory,
            RecursiveMode::Recursive,
        );
        let campaign_list = Arc::new(RwLock::new(HashMap::new()));
        let campaign_broadcaster = Arc::new(Broadcaster::create());
        let campaign_controller = Self {
            campaign_broadcaster: campaign_broadcaster.clone(),
            campaign_list: campaign_list.clone(),
            _watcher: watcher,
        };
        scope.spawn(move |_s| loop {
            match info_struct_receiver.recv() {
                Ok(message) => {
                    CampaignController::reconcile(message, campaign_list.clone());

                    CampaignController::broadcast(
                        campaign_list.clone(),
                        campaign_broadcaster.clone(),
                    );
                }
                Err(_) => break,
            };
        });

        campaign_controller
    }

    /// why we can't just put everything inside the thread
    pub fn get_info_directly(&self) -> HashMap<String, CampaignInfoStruct> {
        self.campaign_list.read().unwrap().deref().clone()
    }

    pub fn get_client(&self) -> Client {
        log::info!("client request");

        let mutex_guard = self.campaign_list.read().unwrap();
        let message = mutex_guard.deref().clone();
        let client = self.campaign_broadcaster.new_client_with_message(&message);
        client
    }

    fn reconcile(
        message: CampaignInfoStruct,
        campaign_list: Arc<RwLock<HashMap<String, CampaignInfoStruct>>>,
    ) {
        campaign_list
            .write()
            .unwrap()
            .insert(message.campaign_name.clone(), message);
    }

    fn broadcast(
        campaign_list: Arc<RwLock<HashMap<String, CampaignInfoStruct>>>,
        broadcaster: Arc<Broadcaster>,
    ) {
        let message = campaign_list.read().unwrap().deref().clone();
        broadcaster.send(&message)
    }
}
