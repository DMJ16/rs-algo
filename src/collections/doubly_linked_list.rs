use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

pub struct ListIterator<T> {
    current: Link<T>,
}

impl<T> ListIterator<T> {
    fn new(start: Link<T>) -> ListIterator<T> {
        ListIterator { current: start }
    }
}

impl<T: Copy> Iterator for ListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value);
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl<T: Copy> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value);
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    pub length: u64,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.length += 1;
        self.tail = Some(new_node);
    }
    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
            }
            None => self.tail = Some(new_node.clone()),
        };
        self.length += 1;
        self.head = Some(new_node);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.length -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head).ok().expect("Error").into_inner().value
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_doubly_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop_front(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.length, 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.length, 0);
        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.length, 2);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.length, 0);
        assert_eq!(list.pop_front(), None)
    }
}
