use std::sync::Arc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn unshift(&self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn shift(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.get_head(), None);

        let list = list.unshift(1).unshift(2).unshift(3);
        assert_eq!(list.get_head(), Some(&3));

        let list = list.shift();
        assert_eq!(list.get_head(), Some(&2));

        let list = list.shift();
        assert_eq!(list.get_head(), Some(&1));

        let list = list.shift();
        assert_eq!(list.get_head(), None);

        let list = list.shift();
        assert_eq!(list.get_head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().unshift(1).unshift(2).unshift(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
