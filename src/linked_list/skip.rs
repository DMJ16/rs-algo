use rand::random;
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    pub fn new(links: Vec<Link>, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: links,
            offset,
            command,
        }))
    }
}

#[derive(Clone)]
pub struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl SkipList {
    pub fn new(max_level: usize) -> SkipList {
        SkipList {
            head: None,
            tails: vec![None; max_level + 1],
            max_level,
            length: 0,
        }
    }

    // pub fn get_level(&self) -> usize {
    //     let mut n = 0;
    //     while random::<bool>() && n < self.max_level {
    //         n +=1;
    //     }
    //     n
    // }

    // pub fn append(&mut self, offset: u64, value: String) {
    //     let new_node = Node::new()
    // }
}
