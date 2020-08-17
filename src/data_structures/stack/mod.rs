pub mod array_stack;

pub trait Stack<T: Clone> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, i: usize) -> T;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}
