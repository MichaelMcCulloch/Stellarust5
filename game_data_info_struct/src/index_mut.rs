use crate::resource;

pub trait IndexMut<T> {
    fn index_mut<'a, 'b>(&'a mut self, res: &'b resource::ResourceClass) -> &'a mut T;
}
