pub mod single_linked_list;

pub trait List<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, x: T);
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize);
}
