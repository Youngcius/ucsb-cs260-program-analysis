use super::domain;
use super::semantics::AbstractSemantics;
use crate::cfg;
use crate::hashset;
use crate::lir;
use crate::lir::Instruction;
use crate::lir::Terminal;
use crate::store;
use crate::utils;
use log;
use std::collections::HashSet;
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

#[derive(Debug, Clone)]
pub struct ReachingDefinitionAnalyzer {
    pub prog: lir::Program,
    pub store: store::ProgramPointStore, // global store involving all variables
    pub solution: HashMap<String, domain::ProgramPoint>, // mapping from program points to program point sets
    pub cfg: cfg::ControlFlowGraph,
    pub pp_def: HashMap<String, Option<lir::Variable>>,
    pub pp_use: HashMap<String, HashSet<lir::Variable>>,
    pub worklist: VecDeque<lir::Block>,
    pub executed: bool,
}

#[derive(Debug, Clone)]
pub struct ControlDependenceAnalyzer {
    pub prog: lir::Program,
    pub cfg: cfg::ControlFlowGraph,
    pub solution: HashMap<String, HashSet<String>>, // mapping from blocks to block sets
    pub executed: bool,
}

pub type ConstantAnalyzer = Analyzer<domain::Constant>;

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
        let mut addrof_ints = prog.get_addrof_ints(func_name);
        // add global to addrof_ints
        for global in &global_ints {
            addrof_ints.push(global.clone());
        }

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

impl ReachingDefinitionAnalyzer {
    pub fn new(prog: lir::Program, func_name: &str) -> Self {
        // Initialized the constant analyzer
        let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
        // let reachable_successors: HashMap<String, Vec<String>> = HashMap::new();
        let mut worklist: VecDeque<lir::Block> = VecDeque::new();

        let mut solution = HashMap::new();
        for bb_label in &cfg.get_all_block_labels() {
            let block = cfg.get_block(bb_label).unwrap();
            for (idx, instr) in block.insts.iter().enumerate() {
                let pp = lir::ProgramPoint {
                    block: bb_label.clone(),
                    location: lir::Location::Instruction(idx),
                    // instr: Some(instr.clone()),
                    // term: None,
                };
                solution.insert(pp.to_string(), domain::ProgramPoint::Bottom);
            }
            let pp = lir::ProgramPoint {
                block: bb_label.clone(),
                location: lir::Location::Terminal,
                // instr: None,
                // term: Some(block.term.clone()),
            };
            solution.insert(pp.to_string(), domain::ProgramPoint::Bottom);
        }

        let mut store = store::ProgramPointStore::new();

        worklist.push_back(cfg.get_entry().unwrap().clone());

        Self {
            prog,
            store,
            solution,
            cfg,
            pp_def: HashMap::new(),
            pp_use: HashMap::new(),
            worklist,
            executed: false,
        }
    }

    fn exe_pp(&mut self, pp: &lir::ProgramPoint) {
        let block = self.cfg.get_block(&pp.block).unwrap();
        match pp.location {
            lir::Location::Instruction(idx) => {
                // let instr = &block.insts[idx];
                match &block.insts[idx] {
                    Instruction::AddrOf { lhs, rhs: _ } => {
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Arith {
                        lhs,
                        aop: _,
                        op1,
                        op2,
                    } => {
                        if let lir::Operand::Var(var) = op1 {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        if let lir::Operand::Var(var) = op2 {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // TODO: update self.solution
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Cmp {
                        lhs,
                        rop: _,
                        op1,
                        op2,
                    } => {
                        if let lir::Operand::Var(var) = op1 {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        if let lir::Operand::Var(var) = op2 {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // TODO: update self.solution
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Copy { lhs, op } => {
                        if let lir::Operand::Var(var) = op {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // TODO: update self.solution
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Alloc { lhs, num, id } => {
                        if let lir::Operand::Var(var) = num {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(id.clone()); // TODO: ?????????
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // TODO: update self.solution
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Gep { lhs, src, idx } => {
                        // get-element-pointer: `x = $gep y 10` takes `y` (which is a pointer to an
                        // array of elements) and assigns to `x` the address of the 10th element of
                        // the array. this is the only way to do pointer arithmetic.
                        if let lir::Operand::Var(var) = idx {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(src.clone());
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // TODO: update self.solution
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Gfp { lhs, src, field } => {
                        // get-field-pointer: `x = $gfp y foo` takes `y` (which is a pointer to a struct)
                        // and assigns to `x` the address of the `foo` field of the struct. the only way to
                        // access fields of a struct is via a pointer.
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(src.clone());
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(field.clone());
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // TODO: update self.solution
                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Load { lhs, src } => {
                        self.pp_def.insert(pp.to_string(), Some(lhs.clone()));
                        // ...... TODO

                        self.store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Store { dst, op } => {
                        // TODO
                    }
                    Instruction::CallExt {
                        lhs,
                        ext_callee,
                        args,
                    } => {
                        // TODO
                    }
                    _ => {}
                }
            }
            lir::Location::Terminal => match &block.term {
                Terminal::Jump(_) => {
                    // TODO: set pp_use to empty???
                    self.pp_def.insert(pp.to_string(), None);
                }
                Terminal::Branch { cond, tt, ff } => {
                    if let lir::Operand::Var(var) = cond {
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(var.clone());
                    }
                    self.pp_def.insert(pp.to_string(), None);
                    // TODO: update self.solution
                }
                Terminal::Ret(ret) => {
                    if let Some(lir::Operand::Var(var)) = ret {
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(var.clone());
                    }
                    self.pp_def.insert(pp.to_string(), None);
                    // TODO: update self.solution
                }
                Terminal::CallDirect {
                    lhs,
                    callee,
                    args,
                    next_bb,
                } => {
                    // TODO
                }
                Terminal::CallIndirect {
                    lhs,
                    callee,
                    args,
                    next_bb,
                } => {
                    // TODO
                }
            },
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

impl AbstractExecution for ReachingDefinitionAnalyzer {
    fn mfp(&mut self) {}

    fn exe_block(&mut self, block: &lir::Block) {
        for (idx, instr) in block.insts.iter().enumerate() {
            let pp = lir::ProgramPoint {
                block: block.id.clone(),
                location: lir::Location::Instruction(idx),
            };
            self.exe_pp(&pp);
        }
        let pp = lir::ProgramPoint {
            block: block.id.clone(),
            location: lir::Location::Terminal,
        };
        self.exe_pp(&pp);
    }

    fn exe_instr(&mut self, _instr: &lir::Instruction, _bb_label: &str) {
        panic!("ReachingDefinitionAnalyzer does not support exe_instr method")
    }

    fn exe_term(&mut self, _term: &lir::Terminal, _bb_label: &str) {
        panic!("ReachingDefinitionAnalyzer does not support exe_term method")
    }
}

pub trait AbstractExecution {
    fn mfp(&mut self);
    fn exe_block(&mut self, block: &lir::Block);
    fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str);
    fn exe_term(&mut self, term: &lir::Terminal, bb_label: &str);
}
