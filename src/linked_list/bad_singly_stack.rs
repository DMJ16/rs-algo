use std::mem;

#[derive(Debug)]
enum Link {
    Nil,
    Cons(Box<Node>),
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: mem::replace(&mut self.head, Link::Nil),
        });
        self.head = Link::Cons(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::Cons(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Nil);

        while let Link::Cons(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None)
    }
}
