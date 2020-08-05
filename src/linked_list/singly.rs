use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Clone, Debug)]
pub struct LinkedList {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(Rc::clone(&new_node)),
            None => self.head = Some(Rc::clone(&new_node)),
        };
        self.length += 1;
        self.tail = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head).ok().expect("Error").into_inner().value
        })
    }
}

// pub enum List {
//     Cons(u32, Box<List>),
//     Nil,
// }

// impl List {
//     pub fn new() -> List {
//         List::Nil
//     }

//     pub fn prepend(self, node: u32) -> List {
//         List::Cons(node, Box::new(self))
//     }

//     pub fn len(&self) -> u32 {
//         match *self {
//             List::Cons(_, ref tail) => 1 + tail.len(),
//             List::Nil => 0,
//         }
//     }

//     pub fn stringify(&self) -> String {
//         match *self {
//             List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
//             List::Nil => "Nil".to_string(),
//         }
//     }
// }
