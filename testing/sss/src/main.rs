use std::{collections::{HashMap, HashSet, VecDeque}, io::Write};

macro_rules! hashset {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };

}

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

    fn find_loop_headers(&self, start_node: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut in_current_path = HashSet::new();
        let mut loop_headers = HashSet::new();
        self.dfs(
            start_node,
            &mut visited,
            &mut stack,
            &mut in_current_path,
            &mut loop_headers,
        );

        for &node in self.edges.keys() {
            if !visited.contains(&node) {
                self.dfs(
                    node,
                    &mut visited,
                    &mut stack,
                    &mut in_current_path,
                    &mut loop_headers,
                );
            }
        }

        loop_headers
    }

    fn dfs(
        &self,
        node: usize,
        visited: &mut HashSet<usize>,
        stack: &mut Vec<usize>,
        in_current_path: &mut HashSet<usize>,
        loop_headers: &mut HashSet<usize>,
    ) {
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

    fn is_edge_in_cycle(&self, start: usize, end: usize) -> bool {
        if self.is_path(start, end) && self.is_path(end, start) {
            return true;
        }
        false
    }

    fn is_path(&self, src: usize, dst: usize) -> bool {
        let mut visited = HashSet::new();
        self.dfs1(src, dst, &mut visited)
    }

    fn dfs1(&self, node: usize, target: usize, visited: &mut HashSet<usize>) -> bool {
        if node == target {
            return true;
        }

        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);

        if let Some(neighbors) = self.edges.get(&node) {
            for &neighbor in neighbors {
                if self.dfs1(neighbor, target, visited) {
                    return true;
                }
            }
        }

        false
    }

    fn get_dominators(&self, start_node: usize) -> HashSet<usize> {
        let mut dominators: HashMap<usize, HashSet<usize>> = HashMap::new();
        let nodes: HashSet<usize> = self.edges.keys().cloned().collect();

        for &node in &nodes {
            dominators.insert(
                node,
                if node == start_node {
                    hashset! {start_node}
                } else {
                    nodes.clone()
                },
            );
        }

        let mut previous_dominators = dominators.clone();

        while {
            for &node in &nodes {
                if node != start_node {
                    let preds: Vec<&usize> = self
                        .edges
                        .iter()
                        .filter(|(_, v)| v.contains(&node))
                        .map(|(k, _)| k)
                        .collect();
                    if !preds.is_empty() {
                        dominators.insert(
                            node,
                            preds
                                .iter()
                                .map(|&&p| previous_dominators[&p].clone())
                                .fold(nodes.clone(), |a, b| a.intersection(&b).cloned().collect()),
                        );
                        dominators.get_mut(&node).unwrap().insert(node);
                    }
                }
            }
            dominators != previous_dominators
        } {
            previous_dominators = dominators.clone();
        }

        dominators[&start_node].clone()
    }

    fn get_all_dominators(&self) -> HashMap<usize, HashSet<usize>> {
        let mut all_dominators = HashMap::new();
        for &node in self.edges.keys() {
            all_dominators.insert(node, self.get_dominators(node));
        }
        all_dominators
    }


    fn to_graphml_file(&self, filename: &str) {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").unwrap();
        file.write_all(b"<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\n").unwrap();
        file.write_all(b"<graph id=\"G\" edgedefault=\"directed\">\n").unwrap();

        for (from, tos) in &self.edges {
            for to in tos {
                file.write_all(format!("<edge source=\"{}\" target=\"{}\"/>\n", from, to).as_bytes()).unwrap();
            }
        }

        file.write_all(b"</graph>\n").unwrap();
        file.write_all(b"</graphml>\n").unwrap();
    }
}

fn test_loop_headers() {
    let mut graph = Graph::new();
    // graph.add_edge(0, 1);
    // graph.add_edge(1, 2);
    // graph.add_edge(2, 1);
    // graph.add_edge(1, 3);
    // graph.add_edge(3, 4);
    // graph.add_edge(4, 5);
    // graph.add_edge(5, 4);
    // graph.add_edge(4, 6);

    /*
    0, 1
    1, 2
    2, 4
    4, 1
    1, 3
    3, 5
    5, 6
    6, 8
    8, 5
    5, 7

     */
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 4);
    graph.add_edge(4, 1);
    graph.add_edge(1, 3);
    graph.add_edge(3, 5);
    graph.add_edge(5, 6);
    graph.add_edge(6, 8);
    graph.add_edge(8, 5);
    graph.add_edge(5, 7);

    let mut count = 0;
    for _ in 0..100 {
        let loop_headers = graph.find_loop_headers(0);
        println!("Loop headers: {:?}", loop_headers);
    }
    println!("Count: {}", count);

    // print edges
    for (from, tos) in &graph.edges {
        for to in tos {
            println!(
                "{} -> {}, in a loop? {}",
                from,
                to,
                graph.is_edge_in_cycle(*from, *to)
            );
        }
    }

    graph.edges.keys().for_each(|k| println!("{}", k));
    graph.to_graphml_file("graph_0.graphml");
}

fn test_get_dominators() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);

    let all_dominators = graph.get_all_dominators();
    println!("Dominators: {:?}", all_dominators);
    graph.to_graphml_file("graph_1.graphml");
}
fn main() {
    test_loop_headers();
    test_get_dominators();
}
