use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
    }

    fn find_loop_headers(&self, start_node: usize) -> HashSet<usize>{
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut in_current_path = HashSet::new();
        let mut loop_headers = HashSet::new();
        self.dfs(start_node, &mut visited, &mut stack, &mut in_current_path, &mut loop_headers);

        for &node in self.edges.keys() {
            if !visited.contains(&node) {
                self.dfs(node, &mut visited, &mut stack, &mut in_current_path, &mut loop_headers);
            }
        }
    
        loop_headers
    }

    fn dfs(&self, node: usize, visited: &mut HashSet<usize>, stack: &mut Vec<usize>, in_current_path: &mut HashSet<usize>, loop_headers: &mut HashSet<usize>) {
        visited.insert(node);
        stack.push(node);
        in_current_path.insert(node);
    
        if let Some(neighbors) = self.edges.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs(neighbor, visited, stack, in_current_path, loop_headers);
                } else if in_current_path.contains(&neighbor) {
                    // Found a back edge, indicating a cycle
                    loop_headers.insert(neighbor);
                }
            }
        }
    
        in_current_path.remove(&node);
        stack.pop();
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 1);
    graph.add_edge(1, 3);
    graph.add_edge(3, 4);
    graph.add_edge(4, 5);
    graph.add_edge(5, 4);
    graph.add_edge(4, 6);

    let mut count = 0;
    for _ in 0..100 {
        let loop_headers = graph.find_loop_headers(0);
        println!("Loop headers: {:?}", loop_headers);
    }
    println!("Count: {}", count);

    // print edges
    for (from, tos) in &graph.edges {
        for to in tos {
            println!("{} -> {}", from, to);
        }
    }
    
    graph.edges.keys().for_each(|k| println!("{}", k));
}
