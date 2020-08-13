pub trait Deque<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn push_back(&mut self, x: T);
    fn push_front(&mut self, x: T);
    fn pop_back(&mut self) -> T;
    fn pop_front(&mut self) -> T;
}
