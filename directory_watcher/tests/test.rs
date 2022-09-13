#[cfg(test)]
mod create {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::mpsc::channel,
    };

    use directory_watcher::{
        create_directory_watcher_and_scan_root, Event, RecursiveMode, Startup,
    };
    use notify::{
        event::{AccessKind, AccessMode},
        EventKind,
    };

    pub struct PathBufWatcher;
    pub struct ScanRoot;
    impl Startup for ScanRoot {
        fn startup(&self, event: &Path) -> Vec<PathBuf> {
            fs::read_dir(event)
                .unwrap()
                .into_iter()
                .map(|d| d.unwrap().path())
                .collect()
        }
    }

    #[test]
    fn create__existing_file__processes_existing_files() {
        let (tx, rx) = channel();
        let scratch = {
            let mut curr = std::env::current_dir().unwrap();
            curr.push("scratch");
            curr
        };

        fs::create_dir(&scratch).unwrap();
        let scratch_file = {
            let mut file = scratch.clone();
            file.push("file.sav");
            file
        };

        std::fs::write(&scratch_file, "").unwrap();

        let _watcher = create_directory_watcher_and_scan_root(
            |e: &Event| -> bool {
                matches!(
                    e,
                    Event {
                        kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
                        paths: _,
                        attrs: _,
                    }
                )
            },
            |path: &Path| -> bool {
                path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with("sav")
            },
            |path: &Path| -> PathBuf { PathBuf::from(path) },
            move |pathbuf| -> () {
                tx.send(pathbuf).unwrap();
            },
            ScanRoot,
            &scratch,
            RecursiveMode::Recursive,
        );

        fs::remove_file(&scratch_file).unwrap();
        fs::remove_dir(&scratch).unwrap();
        println!("{:?}", rx.recv().unwrap());
    }

    #[test]
    fn create__new_file__processes_new_files() {
        let (tx, rx) = channel();
        let scratch = {
            let mut curr = std::env::current_dir().unwrap();
            curr.push("scratch");
            curr
        };

        let scratch_file = {
            let mut file = scratch.clone();
            file.push("file.sav");
            file
        };
        fs::create_dir(&scratch).unwrap();

        let _watcher = create_directory_watcher_and_scan_root(
            |e: &Event| -> bool {
                matches!(
                    e,
                    Event {
                        kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
                        paths: _,
                        attrs: _,
                    }
                )
            },
            |path: &Path| -> bool {
                path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with("sav")
            },
            |path: &Path| -> PathBuf { PathBuf::from(path) },
            move |pathbuf| -> () {
                tx.send(pathbuf).unwrap();
            },
            ScanRoot,
            &scratch,
            RecursiveMode::Recursive,
        );

        std::fs::write(&scratch_file, "").unwrap();

        println!("{:?}", rx.recv().unwrap());
    }
}
