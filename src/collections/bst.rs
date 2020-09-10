use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::Into;

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct BST<T: Copy> {
    root: Child<T>,
}

#[derive(Debug)]
struct Node<T: Copy> {
    val: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T> BST<T>
where
    T: Ord + Copy + Into<i64>,
{
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, val: T) {
        match self.root {
            Some(ref mut root) => root.insert(val),
            None => self.root = Node::new(val),
        }
    }

    pub fn search(&self, val: T) -> bool {
        if let Some(ref root) = self.root {
            root.search(val)
        } else {
            false
        }
    }

    pub fn dfs_in_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.dfs_in_order(&mut result);
        }
        result
    }

    pub fn dfs_pre_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.dfs_pre_order(&mut result);
        }
        result
    }

    pub fn dfs_post_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.dfs_post_order(&mut result);
        }
        result
    }

    pub fn bfs(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.bfs(&mut result);
        }
        result
    }

    pub fn validate(&self) -> bool {
        self.root
            .as_ref()
            .expect("root is undefined")
            .validate(std::i64::MIN, std::i64::MAX)
    }
}

trait TreeNode<T: Copy> {
    fn new(val: T) -> Option<Box<Node<T>>>;
    fn insert(&mut self, val: T);
    fn search(&self, val: T) -> bool;
    fn dfs_in_order(&self, result: &mut Vec<T>);
    fn dfs_pre_order(&self, result: &mut Vec<T>);
    fn dfs_post_order(&self, result: &mut Vec<T>);
    fn bfs(&self, result: &mut Vec<T>);
    fn validate(&self, min: i64, max: i64) -> bool;
}

impl<T> TreeNode<T> for Node<T>
where
    T: Ord + Copy + Into<i64>,
{
    fn new(val: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node {
            val,
            left: None,
            right: None,
        }))
    }

    fn insert(&mut self, val: T) {
        match self.val.cmp(&val) {
            Ordering::Less => match self.right {
                Some(ref mut node) => node.insert(val),
                None => {
                    self.right = Node::new(val);
                }
            },
            Ordering::Greater => match self.left {
                Some(ref mut node) => node.insert(val),
                None => {
                    self.left = Node::new(val);
                }
            },
            Ordering::Equal => {}
        };
    }
    fn search(&self, val: T) -> bool {
        match self.val.cmp(&val) {
            Ordering::Equal => true,
            Ordering::Less => {
                if let Some(ref node) = self.right {
                    node.search(val)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref node) = self.left {
                    node.search(val)
                } else {
                    false
                }
            }
        }
    }
    fn dfs_in_order(&self, result: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.dfs_in_order(result);
        }
        result.push(self.val);
        if let Some(ref right) = self.right {
            right.dfs_in_order(result);
        }
    }

    fn dfs_pre_order(&self, result: &mut Vec<T>) {
        result.push(self.val);
        if let Some(ref left) = self.left {
            left.dfs_pre_order(result);
        }
        if let Some(ref right) = self.right {
            right.dfs_pre_order(result);
        }
    }

    fn dfs_post_order(&self, result: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.dfs_post_order(result);
        }
        if let Some(ref right) = self.right {
            right.dfs_post_order(result);
        }
        result.push(self.val);
    }

    fn bfs(&self, result: &mut Vec<T>) {
        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        let mut current_node = self;
        queue.push_back(current_node);
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                current_node = node;
                result.push(current_node.val);
                if let Some(ref left) = current_node.left {
                    queue.push_back(left);
                }
                if let Some(ref right) = current_node.right {
                    queue.push_back(right);
                }
            }
        }
    }

    fn validate(&self, min: i64, max: i64) -> bool {
        if self.val.into() <= min || self.val.into() >= max {
            return false;
        }
        let left_is_valid = match &self.left {
            Some(left) => left.validate(min, self.val.into()),
            None => true,
        };
        left_is_valid
            && match &self.right {
                Some(right) => right.validate(self.val.into(), max),
                None => true,
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_find_node() {
        let mut tree = BST::new();
        tree.insert(100);
        tree.insert(1);
        tree.insert(600);
        tree.insert(300);
        tree.insert(20);
        tree.insert(5);

        assert_eq!(tree.search(100), true);
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(600), true);
        assert_eq!(tree.search(300), true);
        assert_eq!(tree.search(20), true);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(6), false);
    }

    #[test]
    fn dfs() {
        let mut tree = BST::new();
        tree.insert(100);
        tree.insert(1);
        tree.insert(600);
        tree.insert(300);
        tree.insert(20);
        tree.insert(5);

        let in_order_data = vec![1, 5, 20, 100, 300, 600];
        let pre_order_data = vec![100, 1, 20, 5, 600, 300];
        let post_order_data = vec![5, 20, 1, 300, 600, 100];
        let bfs_data = vec![100, 1, 600, 20, 300, 5];
        println!("{:#?}", tree);
        for i in 0..in_order_data.len() {
            assert_eq!(tree.dfs_in_order()[i], in_order_data[i]);
            assert_eq!(tree.dfs_pre_order()[i], pre_order_data[i]);
            assert_eq!(tree.dfs_post_order()[i], post_order_data[i]);
            assert_eq!(tree.bfs()[i], bfs_data[i]);
        }
    }

    #[test]
    fn validate() {
        let mut tree = BST::new();
        tree.insert(100);
        tree.insert(1);
        tree.insert(600);
        tree.insert(300);
        tree.insert(20);
        tree.insert(5);

        assert_eq!(tree.validate(), true);
    }
}
