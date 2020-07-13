use crate::data_structures::interfaces::*;

pub struct ArrayStack<T> {
    array: Box<[Option<T>]>,
    n: usize,
}

impl<T: Clone> ArrayStack<T> {
    pub fn new(n: usize) -> Self {
        Self {
            array: vec![None; n].into_boxed_slice(),
            n: 0,
        }
    }
}

impl<T: Clone> Stack<T> for ArrayStack<T> {
    fn len(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> T {
        self.array.get(i).unwrap().clone().unwrap()
    }
    fn push(&mut self, x: T) {
        *self.array.get_mut(self.n).unwrap() = Some(x);
        self.n += 1;
    }
    fn pop(&mut self) -> T {
        self.n -= 1;
        self.array.get(self.n).unwrap().clone().unwrap()
    }
}

#[test]
fn test() {
    let mut stack = ArrayStack::new(10);
    stack.push(0);
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.pop(), 1);
    assert_eq!(stack.pop(), 0);
    stack.push(3);
    assert_eq!(stack.pop(), 3);
}
