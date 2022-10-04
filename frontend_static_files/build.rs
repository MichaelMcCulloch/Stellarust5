use static_files::{resource_dir, NpmBuild};

fn main() -> std::io::Result<()> {
    NpmBuild::new("../frontend").install()?.run("build")?;
    resource_dir("../frontend/build").build()
}
