pub mod array_queue;

pub trait Queue<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}
