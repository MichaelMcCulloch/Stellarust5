use directory_watcher::{AccessKind, AccessMode, Event, EventFilter, EventKind, PathFilter};

pub struct CloseWriteFilter;

impl EventFilter for CloseWriteFilter {
    fn filter_event(&self, event: &directory_watcher::Event) -> bool {
        matches! {event, Event { kind: EventKind::Access(AccessKind::Close(AccessMode::Write)), paths, attrs }}
    }
}

pub struct EndsWithSavFilter;

impl PathFilter for EndsWithSavFilter {
    fn filter_path(&self, path: &std::path::Path) -> bool {
        path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with("sav")
    }
}
