use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphErr {
    msg: String,
}
impl GraphErr {
    pub fn new(msg: String) -> Self {
        GraphErr {
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug, Eq)]
pub struct GraphNode<T> {
    vertex: T,
    weight: usize,
}

impl<T: Eq + PartialOrd> Ord for GraphNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Eq> PartialOrd for GraphNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl<T> PartialEq for GraphNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T> GraphNode<T>
where
    T: Eq + Hash + Copy + Ord,
{
    pub fn new(vertex: T, weight: usize) -> Rc<RefCell<Self>> {
        let v = vertex;
        Rc::new(RefCell::new(GraphNode { vertex: v, weight }))
    }
}

#[derive(Debug)]
pub struct Graph<T: Copy> {
    adj_list: HashMap<T, Vec<Rc<RefCell<GraphNode<T>>>>>,
}

pub struct Closure<'s, T> {
    data: T,
    func: &'s dyn Fn(&Closure<T>, &T),
}

impl<T> Graph<T>
where
    T: Eq + Hash + Copy + Debug + Display + Ord,
{
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

    pub fn add_edge(&mut self, v1: T, v2: T, weight: usize) {
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

    pub fn dfs_iter(&self, start: T) -> Vec<T> {
        let mut stack = vec![];
        let mut data = vec![];
        let mut visited = HashMap::new();
        if self.adj_list.contains_key(&start) {
            stack.push(start);
            visited.insert(start, true);
            while let Some(v) = stack.pop() {
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
        data
    }

    pub fn dfs_recur(&self, v: T) -> Vec<T> {
        let data = vec![];
        let mut visited = HashMap::new();

        #[derive(Debug)]
        struct Env<'s, T> {
            v: T,
            data: Vec<T>,
            visited: HashMap<T, bool>,
            adj_list: &'s HashMap<T, Vec<Rc<RefCell<GraphNode<T>>>>>,
        }
        let mut env = Env {
            v,
            data,
            visited,
            adj_list: &self.adj_list,
        };
        fn traverse<T: Eq + Hash + Copy + Debug + Display>(env: &mut Env<T>) {
            if env.adj_list.get(&env.v).unwrap().is_empty() {
                return;
            }
            env.visited.entry(env.v).or_insert(true);
            env.data.push(env.v);
            if env
                .adj_list
                .get(&env.v)
                .unwrap()
                .iter()
                .all(|v| env.visited.contains_key(&v.borrow().vertex))
            {
                return;
            }
            for i in 0..env.adj_list.get(&env.v).unwrap().len() {
                let node = &env.adj_list.get(&env.v).unwrap()[i];
                if env.visited.get(&node.borrow().vertex).is_none() {
                    env.v = node.borrow().vertex;
                    traverse(env)
                }
            }
        };
        traverse(&mut env);
        env.data.to_vec()
    }

    pub fn bfs(&self, start: T) -> Vec<T> {
        let mut queue = VecDeque::new();
        let mut data = vec![];
        let mut visited = HashMap::new();

        if self.adj_list.contains_key(&start) {
            queue.push_back(start);
            visited.insert(start, true);
            while let Some(v) = queue.pop_front() {
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
        data
    }

    pub fn dijkstra(&self, start: T, end: T) -> Vec<T> {
        let mut distances: HashMap<T, usize> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<Rc<RefCell<GraphNode<T>>>>> = BinaryHeap::new();
        let mut prev: HashMap<T, Option<T>> = HashMap::new();
        let mut path: Vec<T> = vec![];
        let mut nearest_vertex: Option<Reverse<Rc<RefCell<GraphNode<T>>>>> = None;
        let mut current_vertex: T = start;

        self.adj_list.keys().for_each(|vertex| {
            if &start == vertex {
                distances.insert(*vertex, 0);
                heap.push(Reverse(GraphNode::new(*vertex, 0)));
            } else {
                distances.insert(*vertex, usize::MAX);
                heap.push(Reverse(GraphNode::new(*vertex, usize::MAX)));
            }
            prev.insert(*vertex, None);
        });
        while !heap.is_empty() {
            nearest_vertex = heap.pop();
            let temp = nearest_vertex.clone();
            current_vertex = temp.unwrap().0.borrow().vertex;
            if current_vertex == end {
                while let Some(Some(temp)) = prev.get(&current_vertex) {
                    path.push(current_vertex);
                    current_vertex = *temp
                }
                break;
            }

            if nearest_vertex.is_some() || distances.get(&current_vertex).unwrap() != &usize::MAX {
                for i in 0..self.adj_list.get(&current_vertex).unwrap().len() {
                    if let Some(temp) = self.adj_list.get(&current_vertex) {
                        let next_node = &temp[i];
                        if let Some(temp) = distances.get(&current_vertex) {
                            let new_dist = temp + next_node.borrow().weight;
                            let next_neighbor_dist =
                                *distances.get(&next_node.borrow().vertex).unwrap();
                            if new_dist < next_neighbor_dist {
                                distances.insert(next_node.borrow().vertex, new_dist);
                                prev.insert(next_node.borrow().vertex, Some(current_vertex));
                                heap.push(Reverse(GraphNode::new(
                                    next_node.borrow().vertex,
                                    new_dist,
                                )));
                            }
                        };
                    }
                }
            }
        }

        path.push(current_vertex);
        path.reverse();
        path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph = Graph::new();
        let empty: Vec<&str> = vec![];
        assert_eq!(graph.dfs_iter("A"), empty);
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
        assert_eq!(graph.dfs_iter("A"), vec!["A", "C", "F", "E", "D", "B"]);
        assert_eq!(graph.dfs_recur("A"), vec!["A", "B", "E", "D", "C", "F",]);
        assert_eq!(graph.bfs("A"), vec!["A", "B", "C", "E", "D", "F"]);
        assert_eq!(graph.dijkstra("A", "E"), vec!["A", "C", "D", "F", "E"]);
    }
}
