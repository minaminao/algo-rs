pub trait List<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, x: T);
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize);
}

pub trait Queue<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}

pub trait Stack<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}

pub trait Deque<T: Clone> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn push_back(&mut self, x: T);
    fn push_front(&mut self, x: T);
    fn pop_back(&mut self) -> T;
    fn pop_front(&mut self) -> T;
}
