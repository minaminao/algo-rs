use super::*;

pub struct ArrayQueue<T> {
    array: Box<[Option<T>]>,
    n: usize,
    head: usize,
    tail: usize,
}

impl<T: Clone> ArrayQueue<T> {
    pub fn new(n: usize) -> Self {
        Self {
            array: vec![None; n].into_boxed_slice(),
            n,
            head: 0,
            tail: 0,
        }
    }
}

impl<T: Clone> Queue<T> for ArrayQueue<T> {
    fn len(&self) -> usize {
        (self.tail + self.n - self.head) % self.n
    }
    fn get(&self, i: usize) -> T {
        self.array
            .get((self.head + i) % self.n)
            .unwrap()
            .clone()
            .unwrap()
    }
    fn push(&mut self, x: T) {
        *self.array.get_mut(self.tail).unwrap() = Some(x);
        self.tail = (self.tail + 1) % self.n;
    }
    fn pop(&mut self) -> T {
        let x = self.array.get(self.head).unwrap().clone().unwrap();
        self.head = (self.head + 1) % self.n;
        x
    }
}

#[test]
fn test() {
    let mut queue = ArrayQueue::new(10);
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.pop(), 2);
    queue.push(3);
    queue.push(4);
    queue.push(5);
    assert_eq!(queue.pop(), 3);
    queue.push(6);
    assert_eq!(queue.pop(), 4);
}
