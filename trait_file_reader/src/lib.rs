use std::path::Path;

pub trait FileReader: Send + 'static {
    type OUT: Send + 'static;
    fn read_file(&self, file: &Path) -> Self::OUT;
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
