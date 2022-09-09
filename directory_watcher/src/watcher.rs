use std::path::{Path, PathBuf};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

pub trait EventFilter: Send + 'static {
    fn filter_event(&self, event: &Event) -> bool;
}
pub trait PathFilter: Send + 'static {
    fn filter_path(&self, path: &Path) -> bool;
}
pub trait FileReader: Send + 'static {
    type OUT: Send + 'static;
    fn read_file(&self, file: &Path) -> Self::OUT;
}
pub trait Delivery<T>: Send + 'static {
    fn deliver(&self, message: T);
}
pub trait Startup: Send + 'static {
    fn startup(&self, start_directory: &Path) -> Vec<PathBuf>;
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
    F: Fn(&Path) -> bool + Send + 'static,
{
    fn filter_path(&self, path: &Path) -> bool {
        (self)(path)
    }
}

impl<F, T: Send + 'static> FileReader for F
where
    F: Fn(&Path) -> T + Send + 'static,
{
    fn read_file(&self, file: &Path) -> T {
        (self)(file)
    }

    type OUT = T;
}

impl<F, T> Delivery<T> for F
where
    F: Fn(T) + Send + 'static,
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

pub fn create_directory_watcher_and_scan_root<
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
    for d in discovered {
        if path_filter.filter_path(&d) {
            let result = file_reader.read_file(d.as_path());

            delivery.deliver(result);
        }
    }
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
