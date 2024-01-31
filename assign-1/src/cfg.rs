use crate::lir;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    // A DAG representing the control flow of a program
    // HashMap<String, lir::Block> is in the same type of lir::Function.body
    // Suppose the node label is the same as the block id
    nodes: HashMap<String, lir::Block>,
    edges: Vec<(String, String)>,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn from_program(prog: &lir::Program) -> Self {
        let mut cfg = Self::new();

        // add dummy entry and exit blocks
        let dummy_entry = lir::Block::new("dummy_entry", &lir::Terminal::Jump("XXX".to_string()));
        cfg.nodes.insert("dummy_entry".to_string(), dummy_entry);
        let dummy_exit = lir::Block::new("dummy_exit", &lir::Terminal::Ret(None));
        cfg.nodes.insert("dummy_exit".to_string(), dummy_exit);

        // insert all blocks in prog into cfg

        // for block in prog.get_blocks() {
        //     cfg.nodes.insert(block.get_id(), block.clone());
        // }

        // for block in prog.get_blocks() {
        //     let block_id = block.get_id();
        //     let next_block_id = block.get_next_block_id();
        //     cfg.edges.push((block_id, next_block_id));
        // }

        // construct dummy "exit" block and insert it to cfg

        // TODO: add edges from all blocks to exit block

        cfg
    }

    pub fn from_function(prog: &lir::Program, func_name: &str) -> Self {
        let mut cfg = Self::new();
        let function = prog.functions.get(func_name).unwrap();
        // function.body.get("entry").unwrap();

        // add dummy entry and exit blocks
        let dummy_entry = lir::Block::new("dummy_entry", &lir::Terminal::Jump("entry".to_string())); // TODO
        cfg.nodes.insert("dummy_entry".to_string(), dummy_entry);
        let dummy_exit = lir::Block::new("dummy_exit", &lir::Terminal::Ret(None));
        cfg.nodes.insert("dummy_exit".to_string(), dummy_exit);

        // insert all blocks in function.body into cfg
        // let blocks: Vec<&lir::Block> = function.body.values().collect();
        for (label, block) in &function.body {
            cfg.nodes.insert(label.clone(), block.clone());
        }

        // add edge: <dummy_entry, entry block>
        cfg.edges
            .push(("dummy_entry".to_string(), "entry".to_string()));

        // construct relationships between blocks in function.body
        for (label, block) in &function.body {
            match block.term {
                lir::Terminal::Jump(ref next) => {
                    cfg.edges.push((label.clone(), next.clone()));
                }
                lir::Terminal::Branch {
                    ref cond,
                    ref tt,
                    ref ff,
                } => {
                    cfg.edges.push((label.clone(), tt.clone()));
                    cfg.edges.push((label.clone(), ff.clone()));
                }

                lir::Terminal::Ret(_) => {
                    // add edge: <block with ret terminal, dummy_exit>
                    cfg.edges.push((label.clone(), "dummy_exit".to_string()));
                }
                _ => {
                    panic!("TODO");
                }
            }
        }
        cfg
    }

    pub fn get_block(&self, label: &str) -> Option<&lir::Block> {
        self.nodes.get(label)
    }

    pub fn get_dummy_entry(&self) -> Option<&lir::Block> {
        self.nodes.get("dummy_entry")
    }

    pub fn get_dummy_exit(&self) -> Option<&lir::Block> {
        self.nodes.get("dummy_exit")
    }

    pub fn get_block_label(&self, block: &lir::Block) -> Option<String> {
        // get the label of a block
        for (label, blk) in &self.nodes {
            if blk == block {
                return Some(label.clone());
            }
        }
        None
    }

    pub fn get_all_block_labels(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }

    pub fn get_predecessor_labels(&self, label: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|(_, dst)| dst == label)
            .map(|(src, _)| src.clone())
            .collect()
    }

    pub fn get_successor_labels(&self, label: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|(src, _)| src == label)
            .map(|(_, dst)| dst.clone())
            .collect()
    }

    pub fn get_predecessors(&self, block: &lir::Block) -> Vec<&lir::Block> {
        let label = self.get_block_label(block).unwrap();
        let predecessor_labels = self.get_predecessor_labels(&label);
        let mut predecessors = Vec::new();
        for predecessor_label in predecessor_labels {
            predecessors.push(self.nodes.get(&predecessor_label).unwrap());
        }
        predecessors
    }

    pub fn get_successors(&self, block: &lir::Block) -> Vec<&lir::Block> {
        let label = self.get_block_label(block).unwrap();
        let successor_labels = self.get_successor_labels(&label);
        let mut successors = Vec::new();
        for successor_label in successor_labels {
            successors.push(self.nodes.get(&successor_label).unwrap());
        }
        successors
    }

    pub fn topological_sort(&self) -> HashMap<String, u32> {
        let mut result = HashMap::new();
        let mut visited = HashSet::new();

        for label in self.nodes.keys() {
            if !visited.contains(label) {
                self.dfs(label, &mut visited, &mut result);
            }
        }

        result
    }

    pub fn dfs(
        &self,
        label: &String,
        visited: &mut HashSet<String>,
        result: &mut HashMap<String, u32>,
    ) {
        visited.insert(label.clone());
        for successor_label in self.get_successor_labels(label) {
            if !visited.contains(&successor_label) {
                self.dfs(&successor_label, visited, result);
            }
        }
        let order = (self.nodes.len() - result.len() - 1) as u32;
        result.insert(label.clone(), order);
    }

    pub fn to_sequence(&self) -> Vec<lir::Block> {
        // convert nodes.values() to a sequence according to topological order of this CFG
        let mut result: Vec<lir::Block> = Vec::new();
        for (label, _) in self.topological_sort() {
            result.push(self.nodes.get(&label).unwrap().clone());
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::super::lir;
    use super::*;

    #[test]
    fn test_topological_sort() {
        let cfg = constr_demo_cfg();

        let result = cfg.topological_sort();
        println!("{:?}", result);
        assert_eq!(result.get("bb0"), Some(&0));
        assert_eq!(result.get("bb5"), Some(&5));

        println!(
            "No.2's successor_labels: {:?}",
            cfg.get_successor_labels("bb2")
        );
    }

    #[test]
    fn test_get_block_id() {
        let cfg = constr_demo_cfg();

        let block0 = cfg.get_block("bb0").unwrap();

        assert_eq!(cfg.get_block_label(&block0), Some("bb0".to_string()));

        println!("{:?}", block0);
    }

    #[test]
    fn test_access_neighbors() {
        let cfg = constr_demo_cfg();
        let block0 = cfg.get_block("bb0").unwrap();
        let block2 = cfg.get_block("bb2").unwrap();
        let block3 = cfg.get_block("bb3").unwrap();
        let block4 = cfg.get_block("bb4").unwrap();

        assert_eq!(cfg.get_predecessor_labels("bb2"), vec!["bb0".to_string()]);
        assert_eq!(
            cfg.get_successor_labels("bb2"),
            vec!["bb3".to_string(), "bb4".to_string()]
        );
        assert_eq!(cfg.get_predecessors(block2), vec![block0]);
        let successors = cfg.get_successors(block2);
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&block3));
        assert!(successors.contains(&block4));
    }

    #[test]
    fn test_example_program() {
        let prog = lir::Program::parse_json("./examples/json/tortoise_and_hare.json");
        // let prog = lir::Program::parse_json("./examples/json/lambda.json");

        println!("number for basic blocks: {}", prog.get_num_basic_blocks());
        // output all blocks
        // for block in prog.get_all_basic_blocks() {
        //     println!("block label: {}", block.id);
        //     println!("\tblock num_instrs: {}", block.insts.len());
        //     println!("\tblock terminal: {:?}", block.term);
        // }
    }

    #[test]
    fn test_example_function() {
        let prog = lir::Program::parse_json("./demos/test3.json");
        let cfg: ControlFlowGraph = ControlFlowGraph::from_function(&prog, "test");

        println!("Testing function: {}", "test");
        // println!("{:#?}", prog);
        println!("topo orders of blocks: {:?}", cfg.topological_sort());
        println!("all block labels: {:?}", cfg.nodes.keys());
        println!("all edges: {:?}", cfg.edges);
        println!("topo orders of blocks: {:?}", cfg.topological_sort());
    }

    fn constr_demo_cfg() -> ControlFlowGraph {
        let mut cfg = ControlFlowGraph::new();

        let block0 = lir::Block::new("block0", &lir::Terminal::Jump("xxx".to_string()));
        let block1 = lir::Block::new("block1", &lir::Terminal::Jump("xxx".to_string()));
        let block2 = lir::Block::new("block2", &lir::Terminal::Jump("xxx".to_string()));
        let block3 = lir::Block::new("block3", &lir::Terminal::Jump("xxx".to_string()));
        let block4 = lir::Block::new("block4", &lir::Terminal::Jump("xxx".to_string()));
        let block5 = lir::Block::new("block5", &lir::Terminal::Jump("xxx".to_string()));

        let label0 = "bb0".to_string();
        let label1 = "bb1".to_string();
        let label2 = "bb2".to_string();
        let label3 = "bb3".to_string();
        let label4 = "bb4".to_string();
        let label5 = "bb5".to_string();

        cfg.nodes.insert(label0.clone(), block0.clone());
        cfg.nodes.insert(label1.clone(), block1.clone());
        cfg.nodes.insert(label2.clone(), block2.clone());
        cfg.nodes.insert(label3.clone(), block3.clone());
        cfg.nodes.insert(label4.clone(), block4.clone());
        cfg.nodes.insert(label5.clone(), block5.clone());

        cfg.edges.push((label0.clone(), label1.clone()));
        cfg.edges.push((label0.clone(), label2.clone()));
        cfg.edges.push((label1.clone(), label3.clone()));
        cfg.edges.push((label2.clone(), label3.clone()));
        cfg.edges.push((label2.clone(), label4.clone()));
        cfg.edges.push((label3.clone(), label5.clone()));
        cfg.edges.push((label4.clone(), label5.clone()));

        cfg
    }
}
