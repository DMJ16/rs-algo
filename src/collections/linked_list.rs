use std::cell::RefCell;
use std::rc::{Rc, Weak};

type StrongLink<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: StrongLink<T>,
    prev: WeakLink<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: StrongLink<T>,
    tail: WeakLink<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.head.take() {
            Some(node) => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: Some(node.clone()),
                    prev: None,
                }));
                let mut m = node.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: None,
                }));
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.tail.take() {
            Some(node) => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    prev: Some(node.clone()),
                    next: None,
                }));
                let strong_node = Weak::upgrade(&node).unwrap();
                let mut m = strong_node.borrow_mut();
                self.tail = Some(Rc::downgrade(&new_node));
                m.next = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: None,
                }));
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T: PartialEq>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let prev_data = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(prev_data))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut node)) => node.push_back(data),
            None => self.push_front(data),
        }
    }
}
