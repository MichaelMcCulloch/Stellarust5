#[cfg(target_os = "linux")]
use directory_watcher::{AccessKind, AccessMode, Event, EventFilter, EventKind, PathFilter};

#[cfg(target_os = "windows")]
use directory_watcher::{Event, EventFilter, EventKind, ModifyKind, PathFilter};

pub struct CloseWriteFilter;
#[cfg(target_os = "linux")]
impl EventFilter for CloseWriteFilter {
    fn filter_event(&self, event: &directory_watcher::Event) -> bool {
        matches! {event, Event { kind: EventKind::Access(AccessKind::Close(AccessMode::Write)), paths: _, attrs: _ }}
    }
}

#[cfg(target_os = "windows")]
impl EventFilter for CloseWriteFilter {
    fn filter_event(&self, event: &directory_watcher::Event) -> bool {
        matches! {event, Event { kind: EventKind::Modify(ModifyKind::Any), paths: _, attrs: _ }}
    }
}

pub struct EndsWithSavFilter;

impl PathFilter for EndsWithSavFilter {
    fn filter_path(&self, path: &std::path::Path) -> bool {
        path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with("sav")
    }
}
