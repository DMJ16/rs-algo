pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> List {
        List::Nil
    }

    pub fn prepend(self, node: u32) -> List {
        List::Cons(node, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => "Nil".to_string(),
        }
    }
}
