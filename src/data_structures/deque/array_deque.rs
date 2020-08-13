use super::*;

pub struct ArrayDeque<T> {
    array: Box<[Option<T>]>,
    n: usize,
    head: usize,
    tail: usize,
}

impl<T: Clone> ArrayDeque<T> {
    pub fn new(n: usize) -> Self {
        Self {
            array: vec![None; n].into_boxed_slice(),
            n,
            head: 0,
            tail: 0,
        }
    }
}

impl<T: Clone> Deque<T> for ArrayDeque<T> {
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
    fn push_back(&mut self, x: T) {
        *self.array.get_mut(self.tail).unwrap() = Some(x);
        self.tail = (self.tail + 1) % self.n;
    }
    fn push_front(&mut self, x: T) {
        self.head = (self.head + self.n - 1) % self.n;
        *self.array.get_mut(self.head).unwrap() = Some(x);
    }
    fn pop_back(&mut self) -> T {
        self.tail = (self.tail + self.n - 1) % self.n;
        self.array.get(self.tail).unwrap().clone().unwrap()
    }
    fn pop_front(&mut self) -> T {
        let x = self.array.get(self.head).unwrap().clone().unwrap();
        self.head = (self.head + 1) % self.n;
        x
    }
}

#[test]
fn test() {
    let mut deque = ArrayDeque::new(10);
    deque.push_back(1);
    deque.push_front(2);
    deque.push_front(3);
    deque.push_front(4);
    // 4 3 2 1
    assert_eq!(deque.pop_front(), 4);
    assert_eq!(deque.pop_back(), 1);
    assert_eq!(deque.pop_back(), 2);
    assert_eq!(deque.pop_back(), 3);
}
