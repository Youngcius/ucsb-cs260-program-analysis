use super::domain;
use super::domain::Number;
use super::semantics::AbstractSemantics;
use crate::cfg;
use crate::lir;
use crate::store;
use crate::utils;
use log;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Analyzer<T> {
    pub prog: lir::Program,
    pub bb2store: HashMap<String, store::Store<T>>,
    pub reachable_successors: HashMap<String, Vec<String>>,
    pub cfg: cfg::ControlFlowGraph,
    pub worklist: VecDeque<lir::Block>,
    pub global_ints: Vec<lir::Variable>,
    pub addrof_ints: Vec<lir::Variable>,
    pub executed: bool,
}

pub type ConstantAnalyzer = Analyzer<domain::Constant>;
pub type IntervalAnalyzer = Analyzer<domain::Interval>;

impl ConstantAnalyzer {
    pub fn new(prog: lir::Program, func_name: &str) -> Self {
        // Initialized the constant analyzer
        let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
        #[cfg(debug_assertions)]
        {
            println!("CFG edges: {:?}", cfg.edges);
        }
        let reachable_successors: HashMap<String, Vec<String>> = HashMap::new();
        let mut worklist: VecDeque<lir::Block> = VecDeque::new();
        let mut bb2store: HashMap<String, store::ConstantStore> = HashMap::new();
        let mut entry_store = store::ConstantStore::new();
        // set content of entry_store
        let global_ints = prog.get_int_globals();
        let param_ints = prog.get_int_parameters(func_name);
        let local_ints = prog.get_int_locals(func_name);
        let addrof_ints = prog.get_addrof_ints(func_name); // addrof_ints includes global_ints

        for local in &local_ints {
            // println!("adding {} to entry_store (BOTTOM)", local.name);
            entry_store.set(local.clone(), domain::Constant::Bottom);
        }
        for global in &global_ints {
            // println!("adding {} to entry_store as (TOP", global.name);
            entry_store.set(global.clone(), domain::Constant::Top);
        }
        for param in &param_ints {
            // println!("adding {} to entry_store (TOP)", param.name);
            entry_store.set(param.clone(), domain::Constant::Top);
        }

        worklist.push_back(cfg.get_entry().unwrap().clone());
        for bb_label in &cfg.get_all_block_labels() {
            bb2store.insert(bb_label.clone(), store::ConstantStore::new());
        }
        bb2store.insert("entry".to_string(), entry_store);

        Self {
            prog,
            bb2store,
            reachable_successors,
            cfg,
            worklist,
            global_ints,
            addrof_ints,
            executed: false,
        }
    }
}

impl IntervalAnalyzer {
    pub fn new(prog: lir::Program, func_name: &str) -> Self {
        // Initialized the constant analyzer
        let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
        let reachable_successors: HashMap<String, Vec<String>> = HashMap::new();
        let mut worklist: VecDeque<lir::Block> = VecDeque::new();
        let mut bb2store: HashMap<String, store::IntervalStore> = HashMap::new();
        let mut entry_store = store::IntervalStore::new();
        // set content of entry_store
        let global_ints = prog.get_int_globals();
        let param_ints = prog.get_int_parameters(func_name);
        let local_ints = prog.get_int_locals(func_name);
        let addrof_ints = prog.get_addrof_ints(func_name); // addrof_ints includes global_ints
        for local in &local_ints {
            entry_store.set(local.clone(), domain::Interval::Bottom);
        }
        for global in &global_ints {
            entry_store.set(global.clone(), domain::Interval::Top);
        }
        for param in &param_ints {
            entry_store.set(param.clone(), domain::Interval::Top);
        }

        worklist.push_back(cfg.get_entry().unwrap().clone());
        for bb_label in &cfg.get_all_block_labels() {
            bb2store.insert(bb_label.clone(), store::IntervalStore::new());
        }
        bb2store.insert("entry".to_string(), entry_store);

        Self {
            prog,
            bb2store,
            reachable_successors,
            cfg,
            worklist,
            global_ints,
            addrof_ints,
            executed: false,
        }
    }
}

impl AbstractExecution for ConstantAnalyzer {
    fn mfp(&mut self) {
        if self.executed {
            log::warn!("Already executed");
            return;
        }
        let mut visited: HashMap<String, u32> = HashMap::new(); // <bb_label, count>
        for bb_label in self.cfg.get_all_block_labels() {
            visited.insert(bb_label.clone(), 0);
        }
        self.executed = true;

        /*
        while !self.worklist.is_empty() {
            let block = self.worklist.pop_front().unwrap();
            self.exe_block(&block);
            visited.insert(block.id.clone(), visited.get(&block.id).unwrap() + 1);

            for succ_label in self.reachable_successors.get(&block.id).unwrap() {
                #[cfg(debug_assertions)]
                {
                    println!(
                        "Joining store {} (just executed) --> store {}",
                        block.id, succ_label
                    );
                    println!("{}", self.bb2store.get(&block.id).unwrap().to_string());
                    println!();
                    println!("{}", self.bb2store.get(succ_label).unwrap().to_string());
                }

                let succ = self.cfg.get_block(&succ_label).unwrap().clone();
                let succ_store = self.bb2store.get(succ_label).unwrap(); // succ_store before joining and executing
                let store_joined = succ_store.join(&self.bb2store.get(&block.id).unwrap());
                let mut new_store = store_joined.clone(); // it may be executed virtually

                if visited.get(&block.id).unwrap() > &1 && visited.get(succ_label).unwrap() > &0 {
                    // it is a block in a loop
                    #[cfg(debug_assertions)]
                    {
                        println!("\n{} is a block in a loop\n", succ_label);
                    }
                    let mut analyzer_duplicate = self.clone();
                    analyzer_duplicate
                        .bb2store
                        .insert(succ_label.clone(), new_store.clone());
                    analyzer_duplicate.exe_block(&succ);
                    new_store = analyzer_duplicate.bb2store.get(succ_label).unwrap().clone();
                }

                if &new_store != succ_store {
                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "\t store {} to be changed (after executing {}), pushed to to worklist",
                            succ_label, block.id
                        );
                    }
                    self.bb2store.insert(succ_label.clone(), store_joined);
                    if !self.worklist.contains(&succ) {
                        self.worklist.push_back(succ.clone());
                    }
                }
            }
        }
        */

        // following is another more direct implementation of the MFP worklist algorithm
        while !self.worklist.is_empty() {
            let block = self.worklist.pop_front().unwrap();
            let store_before = self.bb2store.get(&block.id).unwrap().clone();
            if &block != self.cfg.get_entry().unwrap() {
                let mut store_joined = store::ConstantStore::new();
                for pred in self.cfg.get_predecessors(&block) {
                    match self.reachable_successors.get(&pred.id) {
                        Some(succs) => {
                            if succs.contains(&block.id) {
                                store_joined =
                                    store_joined.join(&self.bb2store.get(&pred.id).unwrap());
                            }
                        }
                        None => {}
                    }
                }
                self.bb2store.insert(block.id.clone(), store_joined);
            }
            self.exe_block(&block);
            let store_after = self.bb2store.get(&block.id).unwrap().clone();
            if store_before != store_after || visited.get(&block.id).unwrap() == &0 {
                for succ_label in self.reachable_successors.get(&block.id).unwrap() {
                    let succ = self.cfg.get_block(&succ_label).unwrap().clone();
                    if !self.worklist.contains(&succ) {
                        self.worklist.push_back(succ.clone());
                    }
                }
            }
            visited.insert(block.id.clone(), visited.get(&block.id).unwrap() + 1);
        }
    }

    fn exe_block(&mut self, block: &lir::Block) {
        #[cfg(debug_assertions)]
        {
            println!("Executing block {}", block.id);
        }
        for instr in &block.insts {
            self.exe_instr(instr, &block.id);
        }
        self.exe_term(&block.term, &block.id);
    }

    fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str) {
        /*
        execute an instruction on the store
        op: Operand::CInt(i32), or Operand::Var { name: String, typ: Type::Int, scope: ...}
        */
        #[cfg(debug_assertions)]
        {
            println!("executing instruction: {:?}", instr);
        }
        let store = self.bb2store.get_mut(bb_label).unwrap();
        match instr {
            lir::Instruction::AddrOf { lhs: _, rhs } => {
                // {"AddrOf": {"lhs": "xxx", "rhs": "xxx"}}
                if let lir::Type::Int = rhs.typ {
                    assert!(self.addrof_ints.contains(rhs));
                    #[cfg(debug_assertions)]
                    {
                        println!("added {} to addrof_ints", rhs.name);
                    }
                }
            }
            lir::Instruction::Alloc {
                lhs: _,
                num: _,
                id: _,
            } => {
                // {"Alloc": {"lhs": "xxx", "num": "xxx", "id": "xxx"}}
            }
            lir::Instruction::Copy { lhs, op } => {
                // {"Copy": {"lhs": "xxx", "op": "xxx"}}
                if let lir::Type::Int = lhs.typ {
                    let res_val: domain::Constant;
                    match op {
                        lir::Operand::Var(var) => {
                            if let lir::Type::Int = var.typ {
                                res_val = store.get(var).unwrap().clone();
                            } else {
                                log::warn!("Copy: lhs and op type mismatch");
                                res_val = domain::Constant::Top;
                            }
                        }
                        lir::Operand::CInt(c) => {
                            res_val = domain::Constant::CInt(*c);
                        }
                    }
                    store.set(lhs.clone(), res_val);
                }
            }
            lir::Instruction::Gep {
                lhs: _,
                src: _,
                idx: _,
            } => {
                // {"Gep": {"lhs": "xxx", "src": "xxx", "idx": "xxx"}}
            }
            lir::Instruction::Arith { lhs, aop, op1, op2 } => {
                // {"Arith": {"lhs": "xxx", "aop": "xxx", "op1": "xxx", "op2": "xxx"}}
                let res_val: domain::Constant;
                match (op1, op2) {
                    (lir::Operand::Var(var1), lir::Operand::Var(var2)) => {
                        if let lir::Type::Int = var1.typ {
                            if let lir::Type::Int = var2.typ {
                                let op1_val = store.get(var1).unwrap();
                                let op2_val = store.get(var2).unwrap();
                                res_val = op1_val.arith(op2_val, aop);
                            } else {
                                res_val = domain::Constant::Top;
                            }
                        } else {
                            res_val = domain::Constant::Top;
                        }
                    }
                    (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
                        if let lir::Type::Int = var.typ {
                            let op1_val = store.get(var).unwrap();
                            let op2_val = domain::Constant::CInt(*c);
                            res_val = op1_val.arith(&op2_val, aop);
                        } else {
                            res_val = domain::Constant::Top;
                        }
                    }
                    (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
                        if let lir::Type::Int = var.typ {
                            let op1_val = domain::Constant::CInt(*c);
                            let op2_val = store.get(var).unwrap();
                            res_val = op1_val.arith(op2_val, aop);
                        } else {
                            res_val = domain::Constant::Top;
                        }
                    }
                    (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
                        let op1_val = domain::Constant::CInt(*c1);
                        let op2_val = domain::Constant::CInt(*c2);
                        res_val = op1_val.arith(&op2_val, aop);
                    }
                }
                store.set(lhs.clone(), res_val);
            }
            lir::Instruction::Load { lhs, src: _ } => {
                // {"Load": {"lhs": "xxx", "src": "xxx"}
                if let lir::Type::Int = lhs.typ {
                    store.set(lhs.clone(), domain::Constant::Top);
                }
            }
            lir::Instruction::Store { dst: _, op } => {
                // {"Store": {"dst": "xxx", "op": "xxx"}}
                // if op is Operand::CInt or in-type Variable, do something
                match op {
                    lir::Operand::CInt(c) => {
                        let op_val = domain::Constant::CInt(*c);
                        let mut new_store = store::ConstantStore::new();
                        for var in self.addrof_ints.iter() {
                            new_store.set(var.clone(), op_val.clone());
                        }
                        #[cfg(debug_assertions)]
                        {
                            println!("Now new_store: {}", new_store.to_string());
                        }
                        #[cfg(debug_assertions)]
                        {
                            println!("In Store instruction, joining store with new_store");
                            println!("Before joining:");
                            println!("{}", store.to_string());
                        }
                        *store = store.join(&new_store);
                        #[cfg(debug_assertions)]
                        {
                            println!("After joining:");
                            println!("{}", store.to_string());
                        }
                    }
                    lir::Operand::Var(var) => {
                        if let lir::Type::Int = var.typ {
                            let op_val = store.get(var).unwrap().clone();
                            let mut new_store = store::ConstantStore::new();
                            for var in self.addrof_ints.iter() {
                                new_store.set(var.clone(), op_val.clone());
                            }
                            #[cfg(debug_assertions)]
                            {
                                println!("Now new_store: {}", new_store.to_string());
                            }
                            #[cfg(debug_assertions)]
                            {
                                println!("In Store instruction, joining store with new_store");
                                println!("Before joining:");
                                println!("{}", store.to_string());
                            }
                            *store = store.join(&new_store);
                            #[cfg(debug_assertions)]
                            {
                                println!("After joining:");
                                println!("{}", store.to_string());
                            }
                        }
                    }
                }
            }
            lir::Instruction::Gfp {
                lhs: _,
                src: _,
                field: _,
            } => {
                // {"Gfp": {"lhs": "xxx", "src": "xxx", "field": "xxx"}}
            }
            lir::Instruction::Cmp { lhs, rop, op1, op2 } => {
                // {"Cmp": {"lhs": "xxx", "rop": "xxx", "op1": "xxx", "op2": "xxx"}}
                #[cfg(debug_assertions)]
                {
                    println!("[CMP] executing instruction: {:?}", instr);
                }
                if let lir::Type::Int = lhs.typ {
                    let res_val: domain::Constant;
                    match (op1, op2) {
                        (lir::Operand::Var(var1), lir::Operand::Var(var2)) => {
                            if let lir::Type::Int = var1.typ {
                                if let lir::Type::Int = var2.typ {
                                    let op1_val = store.get(var1).unwrap();
                                    let op2_val = store.get(var2).unwrap();
                                    res_val = op1_val.cmp(op2_val, rop);
                                } else {
                                    res_val = domain::Constant::Top;
                                }
                            } else {
                                res_val = domain::Constant::Top;
                            }
                        }
                        (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = store.get(var).unwrap();
                                let op2_val = domain::Constant::CInt(*c);
                                res_val = op1_val.cmp(&op2_val, rop);
                            } else {
                                res_val = domain::Constant::Top;
                            }
                        }
                        (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = domain::Constant::CInt(*c);
                                let op2_val = store.get(var).unwrap();
                                res_val = op1_val.cmp(op2_val, rop);
                            } else {
                                res_val = domain::Constant::Top;
                            }
                        }
                        (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
                            let op1_val = domain::Constant::CInt(*c1);
                            let op2_val = domain::Constant::CInt(*c2);
                            res_val = op1_val.cmp(&op2_val, rop);
                        }
                    }
                    store.set(lhs.clone(), res_val);
                }
            }
            lir::Instruction::CallExt {
                lhs,
                ext_callee: _,
                args,
            } => {
                // {"CallExt": {"lhs": "xxx", "ext_callee": "xxx", "args": ["xxx", "xxx"]}}
                // set all global_ints to Top
                for var in self.global_ints.iter() {
                    store.set(var.clone(), domain::Constant::Top);
                }
                // if lhs is int-type Variable, set it to Top
                match lhs {
                    Some(lsh) => {
                        if let lir::Type::Int = lsh.typ {
                            store.set(lsh.clone(), domain::Constant::Top);
                        }
                    }
                    None => {}
                }
                // for any argument that is a pointer able to reach an int-type Variable var, set it to Top
                for arg in args.iter() {
                    if let lir::Operand::Var(var) = arg {
                        if let lir::Type::Pointer(to) = &var.typ {
                            if utils::able_to_reach_int(to) {
                                for var in self.addrof_ints.iter() {
                                    store.set(var.clone(), domain::Constant::Top);
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn exe_term(&mut self, term: &lir::Terminal, bb_label: &str) {
        #[cfg(debug_assertions)]
        {
            println!();
            println!("executing terminal: {:?}", term);
        }
        let store = self.bb2store.get_mut(bb_label).unwrap();
        match term {
            lir::Terminal::CallDirect {
                lhs,
                callee: _,
                args,
                next_bb,
            } => {
                for var in self.global_ints.iter() {
                    store.set(var.clone(), domain::Constant::Top);
                }
                match lhs {
                    Some(lsh) => {
                        if let lir::Type::Int = lsh.typ {
                            store.set(lsh.clone(), domain::Constant::Top);
                        }
                    }
                    None => {}
                }
                for arg in args.iter() {
                    if let lir::Operand::Var(var) = arg {
                        if let lir::Type::Pointer(to) = &var.typ {
                            if utils::able_to_reach_int(to) {
                                for var in self.addrof_ints.iter() {
                                    store.set(var.clone(), domain::Constant::Top);
                                }
                                break;
                            }
                        }
                    }
                }
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![next_bb.clone()]);
            }
            lir::Terminal::CallIndirect {
                lhs,
                callee: _,
                args,
                next_bb,
            } => {
                for var in self.global_ints.iter() {
                    store.set(var.clone(), domain::Constant::Top);
                }
                match lhs {
                    Some(lsh) => {
                        if let lir::Type::Int = lsh.typ {
                            store.set(lsh.clone(), domain::Constant::Top);
                        }
                    }
                    None => {}
                }
                for arg in args.iter() {
                    // println!("traversing arg (op) {:?}", arg);
                    if let lir::Operand::Var(var) = arg {
                        if let lir::Type::Pointer(to) = &var.typ {
                            // println!("traversing arg (ptr) {:?}", arg);
                            if utils::able_to_reach_int(to) {
                                for var in self.addrof_ints.iter() {
                                    store.set(var.clone(), domain::Constant::Top);
                                }
                                break;
                            }
                        }
                    }
                }
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![next_bb.clone()]);
            }
            lir::Terminal::Jump(label) => {
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![label.clone()]);
            }
            lir::Terminal::Branch { cond, tt, ff } => match cond {
                lir::Operand::Var(var) => {
                    if let lir::Type::Int = var.typ {
                        let cond_val = store.get(var).unwrap();
                        if cond_val.is_bottom() {
                            self.reachable_successors
                                .insert(bb_label.to_string(), vec![]);
                        } else if cond_val.is_top() {
                            self.reachable_successors
                                .insert(bb_label.to_string(), vec![tt.clone(), ff.clone()]);
                        } else {
                            if let domain::Constant::CInt(c) = cond_val {
                                if *c == 0 {
                                    self.reachable_successors
                                        .insert(bb_label.to_string(), vec![ff.clone()]);
                                } else {
                                    self.reachable_successors
                                        .insert(bb_label.to_string(), vec![tt.clone()]);
                                }
                            }
                        }
                    }
                }
                lir::Operand::CInt(c) => {
                    if *c == 0 {
                        self.reachable_successors
                            .insert(bb_label.to_string(), vec![ff.clone()]);
                    } else {
                        self.reachable_successors
                            .insert(bb_label.to_string(), vec![tt.clone()]);
                    }
                }
            },
            lir::Terminal::Ret(_) => {
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![]);
            }
        }
    }
}

impl AbstractExecution for IntervalAnalyzer {
    fn mfp(&mut self) {
        if self.executed {
            log::warn!("Already executed");
            return;
        }
        let loop_headers = self.cfg.get_loop_headers();
        // println!("loop headers: {:?}", loop_headers);
        let mut visited: HashMap<String, u32> = HashMap::new(); // <bb_label, count>
        for bb_label in self.cfg.get_all_block_labels() {
            visited.insert(bb_label.clone(), 0);
        }
        self.executed = true;
        while !self.worklist.is_empty() {
            let block = self.worklist.pop_front().unwrap();
            self.exe_block(&block);
            visited.insert(block.id.clone(), visited.get(&block.id).unwrap() + 1);

            for succ_label in self.reachable_successors.get(&block.id).unwrap() {
                #[cfg(debug_assertions)]
                {
                    println!(
                        "Joining store {} (just executed) --> store {}",
                        block.id, succ_label
                    );
                    println!("{}", self.bb2store.get(&block.id).unwrap().to_string());
                    println!();
                    println!("{}", self.bb2store.get(succ_label).unwrap().to_string());
                }

                let succ = self.cfg.get_block(&succ_label).unwrap().clone();
                let succ_store = self.bb2store.get(succ_label).unwrap(); // succ_store before joining and executing
                let store_updated = succ_store.update(&self.bb2store.get(&block.id).unwrap()); // use .update() to update the store
                let store_joined: store::Store<domain::Interval>; // use .join() or .widen() to update the store
                let mut new_store: store::Store<domain::Interval>; // to distinguish if the successor needs to be added to worklist by dummy execution
                if visited.get(succ_label).unwrap() > &0 && loop_headers.contains(succ_label) {
                    // println!("{} is a loop header", succ_label);
                    // println!("{} \n▽\n {}", succ_store.to_string(), self.bb2store.get(&block.id).unwrap().to_string());
                    store_joined = succ_store.widen(&self.bb2store.get(&block.id).unwrap());
                    // println!("After widening: \n{}", store_joined.to_string());
                } else {
                    store_joined = succ_store.join(&self.bb2store.get(&block.id).unwrap());
                }

                if loop_headers.contains(&block.id) {
                    // if current block is a loop header, use store_updated
                    new_store = store_updated.clone();
                } else {
                    // otherwise, use store_joined
                    new_store = store_joined.clone(); // it may be executed virtually
                }

                if visited.get(&block.id).unwrap() > &1 && visited.get(succ_label).unwrap() > &0 {
                    // it is a block in a loop
                    #[cfg(debug_assertions)]
                    {
                        println!("\t{} is a block in a loop", succ_label);
                    }
                    let mut analyzer_duplicate = self.clone();
                    analyzer_duplicate
                        .bb2store
                        .insert(succ_label.clone(), new_store.clone());
                    // println!(">>>>>>>>>>>>>>>>>>>>>>>> in duplicate");
                    analyzer_duplicate.exe_block(&succ);
                    // println!("in duplicate <<<<<<<<<<<<<<<<<<<<<<<<");
                    new_store = analyzer_duplicate.bb2store.get(succ_label).unwrap().clone();
                }

                if &new_store != succ_store {
                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "\tstore {} to be changed (after executing {}), pushed to worklist",
                            succ_label, block.id
                        );
                    }
                    if loop_headers.contains(&block.id) {
                        // if current block is a loop header, use store_updated
                        self.bb2store.insert(succ_label.clone(), store_updated);
                    } else {
                        // otherwise, use store_joined
                        self.bb2store.insert(succ_label.clone(), store_joined);
                    }

                    if !self.worklist.contains(&succ) {
                        // avoiding multiple consecutive executions on the same block
                        self.worklist.push_back(succ.clone());
                    }
                }
            }
        }
    }

    fn exe_block(&mut self, block: &lir::Block) {
        #[cfg(debug_assertions)]
        {
            println!();
            println!("Executing block {}", block.id);
        }
        for instr in &block.insts {
            self.exe_instr(instr, &block.id);
        }
        self.exe_term(&block.term, &block.id);
    }

    fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str) {
        /*
        execute an instruction on the store
        op: Operand::CInt(i32), or Operand::Var { name: String, typ: Type::Int, scope: ...}
        */
        #[cfg(debug_assertions)]
        {
            println!("executing instruction: {:?}", instr);
        }
        let store = self.bb2store.get_mut(bb_label).unwrap();
        match instr {
            lir::Instruction::AddrOf { lhs: _, rhs } => {
                // {"AddrOf": {"lhs": "xxx", "rhs": "xxx"}}
                if let lir::Type::Int = rhs.typ {
                    assert!(self.addrof_ints.contains(rhs));
                }
            }
            lir::Instruction::Alloc {
                lhs: _,
                num: _,
                id: _,
            } => {
                // {"Alloc": {"lhs": "xxx", "num": "xxx", "id": "xxx"}}
            }
            lir::Instruction::Copy { lhs, op } => {
                // {"Copy": {"lhs": "xxx", "op": "xxx"}}
                if let lir::Type::Int = lhs.typ {
                    let res_val: domain::Interval;
                    match op {
                        lir::Operand::Var(var) => {
                            if let lir::Type::Int = var.typ {
                                res_val = store.get(var).unwrap().clone();
                            } else {
                                log::warn!("Copy: lhs and op type mismatch");
                                res_val = domain::Interval::Top;
                            }
                        }
                        lir::Operand::CInt(c) => {
                            res_val =
                                domain::Interval::Range(Number::Integer(*c), Number::Integer(*c));
                        }
                    }
                    store.set(lhs.clone(), res_val);
                }
            }
            lir::Instruction::Gep {
                lhs: _,
                src: _,
                idx: _,
            } => {
                // {"Gep": {"lhs": "xxx", "src": "xxx", "idx": "xxx"}}
            }
            lir::Instruction::Arith { lhs, aop, op1, op2 } => {
                // {"Arith": {"lhs": "xxx", "aop": "xxx", "op1": "xxx", "op2": "xxx"}}
                let res_val: domain::Interval;
                match (op1, op2) {
                    (lir::Operand::Var(var1), lir::Operand::Var(var2)) => {
                        if let lir::Type::Int = var1.typ {
                            if let lir::Type::Int = var2.typ {
                                let op1_val = store.get(var1).unwrap();
                                let op2_val = store.get(var2).unwrap();
                                res_val = op1_val.arith(op2_val, aop);
                            } else {
                                res_val = domain::Interval::Top;
                            }
                        } else {
                            res_val = domain::Interval::Top;
                        }
                    }
                    (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
                        if let lir::Type::Int = var.typ {
                            let op1_val = store.get(var).unwrap();
                            let op2_val =
                                domain::Interval::Range(Number::Integer(*c), Number::Integer(*c));
                            res_val = op1_val.arith(&op2_val, aop);
                        } else {
                            res_val = domain::Interval::Top;
                        }
                    }
                    (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
                        if let lir::Type::Int = var.typ {
                            let op1_val =
                                domain::Interval::Range(Number::Integer(*c), Number::Integer(*c));
                            let op2_val = store.get(var).unwrap();
                            res_val = op1_val.arith(op2_val, aop);
                        } else {
                            res_val = domain::Interval::Top;
                        }
                    }
                    (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
                        let op1_val =
                            domain::Interval::Range(Number::Integer(*c1), Number::Integer(*c1));
                        let op2_val =
                            domain::Interval::Range(Number::Integer(*c2), Number::Integer(*c2));
                        res_val = op1_val.arith(&op2_val, aop);
                    }
                }
                store.set(lhs.clone(), res_val);
            }
            lir::Instruction::Load { lhs, src: _ } => {
                // {"Load": {"lhs": "xxx", "src": "xxx"}
                if let lir::Type::Int = lhs.typ {
                    store.set(lhs.clone(), domain::Interval::Top);
                }
            }
            lir::Instruction::Store { dst: _, op } => {
                // {"Store": {"dst": "xxx", "op": "xxx"}}
                // if op is Operand::CInt or in-type Variable, do something
                match op {
                    lir::Operand::CInt(c) => {
                        let op_val =
                            domain::Interval::Range(Number::Integer(*c), Number::Integer(*c));
                        let mut new_store = store::IntervalStore::new();
                        for var in self.addrof_ints.iter() {
                            new_store.set(var.clone(), op_val.clone());
                        }
                        *store = store.join(&new_store);
                    }
                    lir::Operand::Var(var) => {
                        if let lir::Type::Int = var.typ {
                            let op_val = store.get(var).unwrap().clone();
                            let mut new_store = store::IntervalStore::new();
                            for var in self.addrof_ints.iter() {
                                new_store.set(var.clone(), op_val.clone());
                            }
                            *store = store.join(&new_store);
                        }
                    }
                }
            }
            lir::Instruction::Gfp {
                lhs: _,
                src: _,
                field: _,
            } => {
                // {"Gfp": {"lhs": "xxx", "src": "xxx", "field": "xxx"}}
            }
            lir::Instruction::Cmp { lhs, rop, op1, op2 } => {
                // {"Cmp": {"lhs": "xxx", "rop": "xxx", "op1": "xxx", "op2": "xxx"}}
                #[cfg(debug_assertions)]
                {
                    println!("[CMP] executing instruction: {:?}", instr);
                }
                if let lir::Type::Int = lhs.typ {
                    let res_val: domain::Interval;
                    match (op1, op2) {
                        (lir::Operand::Var(var1), lir::Operand::Var(var2)) => {
                            if let lir::Type::Int = var1.typ {
                                if let lir::Type::Int = var2.typ {
                                    let op1_val = store.get(var1).unwrap();
                                    let op2_val = store.get(var2).unwrap();
                                    res_val = op1_val.cmp(op2_val, rop);
                                } else {
                                    res_val = domain::UNDECIDED_INTERVAL;
                                }
                            } else {
                                res_val = domain::UNDECIDED_INTERVAL;
                            }
                        }
                        (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = store.get(var).unwrap();
                                let op2_val = domain::Interval::Range(
                                    Number::Integer(*c),
                                    Number::Integer(*c),
                                );
                                res_val = op1_val.cmp(&op2_val, rop);
                            } else {
                                res_val = domain::UNDECIDED_INTERVAL;
                            }
                        }
                        (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = domain::Interval::Range(
                                    Number::Integer(*c),
                                    Number::Integer(*c),
                                );
                                let op2_val = store.get(var).unwrap();
                                res_val = op1_val.cmp(op2_val, rop);
                                // println!("op1_val: {}, op2_val: {}, res_val: {}", op1_val, op2_val, res_val);
                            } else {
                                res_val = domain::UNDECIDED_INTERVAL;
                            }
                        }
                        (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
                            let op1_val =
                                domain::Interval::Range(Number::Integer(*c1), Number::Integer(*c1));
                            let op2_val =
                                domain::Interval::Range(Number::Integer(*c2), Number::Integer(*c2));
                            res_val = op1_val.cmp(&op2_val, rop);
                        }
                    }
                    store.set(lhs.clone(), res_val);
                }
            }
            lir::Instruction::CallExt {
                lhs,
                ext_callee: _,
                args,
            } => {
                // {"CallExt": {"lhs": "xxx", "ext_callee": "xxx", "args": ["xxx", "xxx"]}}
                // set all global_ints to Top
                for var in self.global_ints.iter() {
                    store.set(var.clone(), domain::Interval::Top);
                }
                // if lhs is int-type Variable, set it to Top
                match lhs {
                    Some(lsh) => {
                        if let lir::Type::Int = lsh.typ {
                            store.set(lsh.clone(), domain::Interval::Top);
                        }
                    }
                    None => {}
                }
                // for any argument that is a pointer able to reach an int-type Variable var, set it to Top
                for arg in args.iter() {
                    if let lir::Operand::Var(var) = arg {
                        if let lir::Type::Pointer(to) = &var.typ {
                            if utils::able_to_reach_int(to) {
                                for var in self.addrof_ints.iter() {
                                    store.set(var.clone(), domain::Interval::Top);
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn exe_term(&mut self, term: &lir::Terminal, bb_label: &str) {
        let store = self.bb2store.get_mut(bb_label).unwrap();
        match term {
            lir::Terminal::CallDirect {
                lhs,
                callee: _,
                args,
                next_bb,
            } => {
                for var in self.global_ints.iter() {
                    store.set(var.clone(), domain::Interval::Top);
                }
                match lhs {
                    Some(lsh) => {
                        if let lir::Type::Int = lsh.typ {
                            store.set(lsh.clone(), domain::Interval::Top);
                        }
                    }
                    None => {}
                }
                for arg in args.iter() {
                    if let lir::Operand::Var(var) = arg {
                        if let lir::Type::Pointer(to) = &var.typ {
                            if utils::able_to_reach_int(to) {
                                for var in self.addrof_ints.iter() {
                                    store.set(var.clone(), domain::Interval::Top);
                                }
                                break;
                            }
                        }
                    }
                }
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![next_bb.clone()]);
            }
            lir::Terminal::CallIndirect {
                lhs,
                callee: _,
                args,
                next_bb,
            } => {
                for var in self.global_ints.iter() {
                    store.set(var.clone(), domain::Interval::Top);
                }
                match lhs {
                    Some(lsh) => {
                        if let lir::Type::Int = lsh.typ {
                            store.set(lsh.clone(), domain::Interval::Top);
                        }
                    }
                    None => {}
                }
                for arg in args.iter() {
                    if let lir::Operand::Var(var) = arg {
                        if let lir::Type::Pointer(to) = &var.typ {
                            if utils::able_to_reach_int(to) {
                                for var in self.addrof_ints.iter() {
                                    store.set(var.clone(), domain::Interval::Top);
                                }
                                break;
                            }
                        }
                    }
                }
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![next_bb.clone()]);
            }
            lir::Terminal::Jump(label) => {
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![label.clone()]);
            }
            lir::Terminal::Branch { cond, tt, ff } => match cond {
                lir::Operand::Var(var) => {
                    if let lir::Type::Int = var.typ {
                        let cond_val = store.get(var).unwrap();
                        match cond_val {
                            domain::Interval::Bottom => {
                                self.reachable_successors
                                    .insert(bb_label.to_string(), vec![]);
                            }
                            domain::Interval::Top => {
                                self.reachable_successors
                                    .insert(bb_label.to_string(), vec![tt.clone(), ff.clone()]);
                            }
                            domain::Interval::Range(l, u) => {
                                if l == u {
                                    if *l == Number::Integer(0) {
                                        self.reachable_successors
                                            .insert(bb_label.to_string(), vec![ff.clone()]);
                                    } else {
                                        self.reachable_successors
                                            .insert(bb_label.to_string(), vec![tt.clone()]);
                                    }
                                } else if *l <= Number::Integer(0) && *u >= Number::Integer(0) {
                                    self.reachable_successors
                                        .insert(bb_label.to_string(), vec![tt.clone(), ff.clone()]);
                                } else {
                                    self.reachable_successors
                                        .insert(bb_label.to_string(), vec![tt.clone()]);
                                }
                            }
                        }
                    }
                }
                lir::Operand::CInt(c) => {
                    if *c == 0 {
                        self.reachable_successors
                            .insert(bb_label.to_string(), vec![ff.clone()]);
                    } else {
                        self.reachable_successors
                            .insert(bb_label.to_string(), vec![tt.clone()]);
                    }
                }
            },
            lir::Terminal::Ret(_) => {
                self.reachable_successors
                    .insert(bb_label.to_string(), vec![]);
            }
        }
    }
}

pub trait AbstractExecution {
    fn mfp(&mut self);
    fn exe_block(&mut self, block: &lir::Block);
    fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str);
    fn exe_term(&mut self, term: &lir::Terminal, bb_label: &str);
}
