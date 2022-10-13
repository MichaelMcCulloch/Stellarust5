use crate::resource;

pub trait Index<T> {
    fn index<'a, 'b>(&'a self, res: &'b resource::ResourceClass) -> &'a T;
}
