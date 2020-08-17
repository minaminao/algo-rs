use super::*;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: Option<T>,
    next: Link<T>,
}

pub struct SinglyLinkedList<T> {
    n: usize,
    head: Link<T>,
    //tail: Link<T>, // TODO
}

impl<T: Clone> SinglyLinkedList<T> {
    pub fn new() -> Self {
        let dummy = Node {
            elem: None,
            next: None,
        };
        Self {
            n: 0,
            head: Some(Rc::new(RefCell::new(dummy))),
        }
    }
}

impl<T: Clone> List<T> for SinglyLinkedList<T> {
    fn len(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> T {
        let mut node = self.head.clone();
        for _ in 0..=i {
            node = node.unwrap().borrow().next.clone();
        }
        node.unwrap().borrow().elem.clone().unwrap()
    }
    fn set(&mut self, i: usize, x: T) {
        let mut node = self.head.clone();
        for _ in 0..i {
            node = node.unwrap().borrow().next.clone();
        }
        node.unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow_mut()
            .elem = Some(x);
    }
    fn add(&mut self, i: usize, elem: T) {
        let mut node = self.head.clone();
        for _ in 0..i {
            node = node.unwrap().borrow().next.clone();
        }
        let new_node = Node {
            elem: Some(elem),
            next: node.clone().unwrap().borrow().next.clone(),
        };
        node.unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
        self.n += 1;
    }
    fn remove(&mut self, i: usize) {
        let mut node = self.head.clone();
        for _ in 0..i {
            node = node.unwrap().borrow().next.clone();
        }
        let next_node = node
            .clone()
            .unwrap()
            .borrow()
            .next
            .clone()
            .unwrap()
            .borrow()
            .next
            .clone();
        node.unwrap().borrow_mut().next = next_node;
        self.n -= 1;
    }
}

// TODO
// impl<T> Drop for SinglyLinkedList<T> {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, None);

//         while let Some(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, None);
//         }
//     }
// }

#[test]
fn test() {
    let mut list = SinglyLinkedList::new();
    assert_eq!(list.len(), 0);
    list.add(0, 0);
    assert_eq!(list.len(), 1);
    assert_eq!(list.get(0), 0);
    list.add(0, 0);
    list.add(0, 1);
    list.add(0, 2);
    assert_eq!(list.len(), 4);
    assert_eq!(list.get(0), 2);
    assert_eq!(list.get(1), 1);
    assert_eq!(list.get(2), 0);
    assert_eq!(list.get(3), 0);
    list.set(1, 3);
    assert_eq!(list.len(), 4);
    assert_eq!(list.get(0), 2);
    assert_eq!(list.get(1), 3);
    assert_eq!(list.get(2), 0);
    assert_eq!(list.get(3), 0);
    list.remove(1);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get(0), 2);
    assert_eq!(list.get(1), 0);
    assert_eq!(list.get(2), 0);
}
