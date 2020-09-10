use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphErr {
    msg: String,
}
impl GraphErr {
    pub fn new(msg: &str) -> Self {
        GraphErr {
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct GraphNode<T> {
    vertex: T,
    weight: u32,
}

impl<T: Eq + Hash + Copy> GraphNode<T> {
    pub fn new(vertex: T, weight: u32) -> Rc<RefCell<Self>> {
        let v = vertex;
        Rc::new(RefCell::new(GraphNode { vertex: v, weight }))
    }
}

#[derive(Debug)]
pub struct Graph<T> {
    adj_list: HashMap<T, Vec<Rc<RefCell<GraphNode<T>>>>>,
}

impl<T: Eq + Hash + Copy> Graph<T> {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: T) {
        let v = vertex;
        if self.adj_list.get(&v).is_none() {
            self.adj_list.insert(v, vec![]);
        }
    }

    pub fn add_edge(&mut self, v1: T, v2: T, weight: u32) {
        if self.adj_list.contains_key(&v1) && self.adj_list.contains_key(&v2) {
            match self.adj_list.get_mut(&v1) {
                Some(vec) => vec.push(GraphNode::new(v2, weight)),
                None => println!("invalid node vector"),
            }
            match self.adj_list.get_mut(&v2) {
                Some(vec) => vec.push(GraphNode::new(v1, weight)),
                None => println!("invalid node vector"),
            }
        }
    }

    pub fn dfs(&self, start_v: T) -> Vec<T> {
        let mut stack = vec![];
        let mut data = vec![];
        let mut visited = HashMap::new();
        if self.adj_list.contains_key(&start_v) {
            stack.push(start_v);
            visited.insert(start_v, true);
            while !stack.is_empty() {
                if let Some(v) = stack.pop() {
                    data.push(v);
                    match self.adj_list.get(&v) {
                        Some(edges) => {
                            for edge in edges {
                                if !visited.contains_key(&edge.borrow().vertex) {
                                    visited.entry(edge.borrow().vertex).or_insert(true);
                                    stack.push(edge.borrow().vertex);
                                }
                            }
                        }
                        None => println!("invalid edges vector"),
                    }
                }
            }
        }
        data
    }

    pub fn bfs(&self, start_v: T) -> Vec<T> {
        let mut queue = VecDeque::new();
        let mut data = vec![];
        let mut visited = HashMap::new();
        if self.adj_list.contains_key(&start_v) {
            queue.push_back(start_v);
            visited.insert(start_v, true);
            while !queue.is_empty() {
                if let Some(v) = queue.pop_front() {
                    data.push(v);
                    match self.adj_list.get(&v) {
                        Some(edges) => {
                            for edge in edges {
                                if !visited.contains_key(&edge.borrow().vertex) {
                                    visited.entry(edge.borrow().vertex).or_insert(true);
                                    queue.push_back(edge.borrow().vertex);
                                }
                            }
                        }
                        None => println!("invalid edges vector"),
                    }
                }
            }
        }
        data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph = Graph::new();
        let empty: Vec<&str> = vec![];
        assert_eq!(graph.dfs("A"), empty);
        assert_eq!(graph.bfs("A"), empty);

        graph.add_vertex("A");
        graph.add_vertex("B");
        graph.add_vertex("C");
        graph.add_vertex("D");
        graph.add_vertex("E");
        graph.add_vertex("F");
        graph.add_edge("A", "B", 4);
        graph.add_edge("A", "C", 2);
        graph.add_edge("B", "E", 3);
        graph.add_edge("C", "D", 2);
        graph.add_edge("C", "F", 4);
        graph.add_edge("D", "E", 3);
        graph.add_edge("D", "F", 1);
        graph.add_edge("E", "F", 1);
        assert_eq!(graph.dfs("A"), vec!["A", "C", "F", "E", "D", "B"]);
        assert_eq!(graph.bfs("A"), vec!["A", "B", "C", "E", "D", "F"]);
    }
}
