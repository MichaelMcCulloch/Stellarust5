use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::path::{Path, PathBuf};

pub trait EventFilter: Send + 'static {
    fn filter_event(&self, event: &Event) -> bool;
}
pub trait PathFilter: Sync + Send + 'static {
    fn filter_path(&self, path: &Path) -> bool;
}

pub trait Delivery<T>: Sync + Send + 'static {
    fn deliver(&self, message: T);
}
pub trait Startup: Send + 'static {
    fn startup(&self, start_directory: &Path) -> Vec<PathBuf>;
}
pub trait FileReader: Sync + Send + 'static {
    type OUT: Send + 'static;
    fn read_file(&self, file: &Path) -> Self::OUT;
}
impl<F, T: Send + 'static> FileReader for F
where
    F: Fn(&Path) -> T + Sync + Send + 'static,
{
    fn read_file(&self, file: &Path) -> T {
        (self)(file)
    }

    type OUT = T;
}

impl<F> EventFilter for F
where
    F: Fn(&Event) -> bool + Send + 'static,
{
    fn filter_event(&self, event: &Event) -> bool {
        (self)(event)
    }
}
impl<F> PathFilter for F
where
    F: Fn(&Path) -> bool + Send + Sync + 'static,
{
    fn filter_path(&self, path: &Path) -> bool {
        (self)(path)
    }
}

impl<F, T> Delivery<T> for F
where
    F: Fn(T) + Sync + Send + 'static,
{
    fn deliver(&self, message: T) {
        (self)(message)
    }
}

impl<F> Startup for F
where
    F: Fn(&Path) -> Vec<PathBuf> + Send + 'static,
{
    fn startup(&self, start_directory: &Path) -> Vec<PathBuf> {
        (self)(start_directory)
    }
}

/// Wrap the notify.rs based RecommendedWatcher in some behavior, including
/// - How to gather files on on startup
/// - How to filter events
/// - How to filter files
/// - How to process the file
/// - How to deliver the processed result
pub trait DirectoryWatcher {
    fn create_directory_watcher_and_scan_root<
        T,
        E: EventFilter,
        P: PathFilter,
        R: FileReader<OUT = T>,
        D: Delivery<T>,
        S: Startup,
        Dir: AsRef<Path>,
    >(
        event_filter: E,
        path_filter: P,
        file_reader: R,
        delivery: D,
        startup: S,
        directory: &Dir,
        recursive_mode: RecursiveMode,
    ) -> RecommendedWatcher;
}

pub struct DefaultWatcher;

impl DirectoryWatcher for DefaultWatcher {
    fn create_directory_watcher_and_scan_root<
        T,
        E: EventFilter,
        P: PathFilter,
        R: FileReader<OUT = T>,
        D: Delivery<T>,
        S: Startup,
        Dir: AsRef<Path>,
    >(
        event_filter: E,
        path_filter: P,
        file_reader: R,
        delivery: D,
        startup: S,
        directory: &Dir,
        recursive_mode: RecursiveMode,
    ) -> RecommendedWatcher {
        let discovered = startup.startup(directory.as_ref());

        discovered
            .par_iter()
            .filter(|path| path_filter.filter_path(path))
            .for_each(|path| {
                let result = file_reader.read_file(path);
                delivery.deliver(result);
            });
        let event_handler = move |event: Result<Event, notify::Error>| -> () {
            match event {
                Ok(event) => {
                    if event_filter.filter_event(&event) {
                        let paths = event.paths;
                        for path in paths {
                            if path_filter.filter_path(&path) {
                                let output = file_reader.read_file(&path);

                                delivery.deliver(output);
                            }
                        }
                    }
                }
                Err(_) => {}
            };
        };
        let mut watcher = RecommendedWatcher::new(event_handler, Config::default()).unwrap();

        watcher.watch(directory.as_ref(), recursive_mode).unwrap();

        watcher
    }
}
