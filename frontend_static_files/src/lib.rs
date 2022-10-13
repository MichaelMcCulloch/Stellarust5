use static_files::Resource;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn generate_static_files() -> HashMap<&'static str, Resource> {
    generate()
}
