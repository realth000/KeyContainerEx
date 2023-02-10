#[derive(Debug)]
pub struct ObjectList<T> {
    list: Vec<T>,
}

impl<T> ObjectList<T> {
    pub const fn new() -> ObjectList<T> {
        ObjectList { list: Vec::new() }
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }
}
