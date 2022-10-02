use std::collections::HashMap;

use static_files::Resource;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn generate_static_files() -> HashMap<&'static str, Resource> {
    generate()
}
