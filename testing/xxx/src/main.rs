use std::collections::{HashMap, HashSet};

// 定义图节点结构体
#[derive(Debug)]
struct CFGNode {
    id: usize,
    successors: Vec<usize>,
}

// 定义图结构体
struct ControlFlowGraph {
    nodes: HashMap<usize, CFGNode>,
}

impl ControlFlowGraph {
    fn new() -> Self {
        ControlFlowGraph {
            nodes: HashMap::new(),
        }
    }

    // 添加节点
    fn add_node(&mut self, id: usize, successors: Vec<usize>) {
        let node = CFGNode { id, successors };
        self.nodes.insert(id, node);
    }

    // 计算支配树
    fn compute_dominator_tree(&self, start_node_id: usize) -> HashMap<usize, usize> {
        let mut dominators = HashMap::new();
        let mut visited = HashSet::new();

        // 初始化支配者为自己
        dominators.insert(start_node_id, start_node_id);

        // DFS遍历
        self.dfs(start_node_id, &mut dominators, &mut visited);

        dominators
    }

    // DFS辅助函数
    fn dfs(&self, current_node_id: usize, dominators: &mut HashMap<usize, usize>, visited: &mut HashSet<usize>) {
        visited.insert(current_node_id);

        if let Some(current_node) = self.nodes.get(&current_node_id) {
            for &successor_id in &current_node.successors {
                if !visited.contains(&successor_id) {
                    dominators.insert(successor_id, current_node_id);
                    self.dfs(successor_id, dominators, visited);
                }
            }
        }
    }
}

fn main() {
    // 创建一个简单的控制流图
    let mut cfg = ControlFlowGraph::new();
    cfg.add_node(1, vec![2]);
    cfg.add_node(2, vec![3, 4]);
    cfg.add_node(3, vec![5]);
    cfg.add_node(4, vec![5]);

    // 计算支配树
    let start_node_id = 1;
    let dominators = cfg.compute_dominator_tree(start_node_id);

    // 打印支配关系
    println!("Dominator Tree:");
    for (node_id, dominator_id) in dominators {
        println!("Node {} is dominated by Node {}", node_id, dominator_id);
    }
}
