use super::domain;
use super::semantics::AbstractSemantics;
use crate::cfg;
use crate::hashset;
use crate::lir;
use crate::lir::Instruction;
use crate::lir::Terminal;
use crate::store;
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
    // pub store: store::ProgramPointStore, // global store involving all variables
    pub bb2store: HashMap<String, store::ProgramPointStore>, // mapping from block to store
    pub cfg: cfg::ControlFlowGraph,
    pub worklist: VecDeque<lir::Block>,
    pub solution: HashMap<String, domain::ProgramPoint>, // mapping from program points to program point sets
    pub pp_def: HashMap<String, HashSet<lir::Variable>>,
    pub pp_use: HashMap<String, HashSet<lir::Variable>>,
    pub addr_taken: Vec<lir::Variable>,
    pub executed: bool,
}

#[derive(Debug, Clone)]
pub struct ControlDependenceAnalyzer {
    pub prog: lir::Program,
    pub cfg: cfg::ControlFlowGraph,
    pub solution: HashMap<String, HashSet<String>>, // mapping from blocks to block sets
    pub executed: bool,
}

impl ReachingDefinitionAnalyzer {
    pub fn new(prog: lir::Program, func_name: &str) -> Self {
        // Initialized the reaching definition analyzer
        let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
        // let reachable_successors: HashMap<String, Vec<String>> = HashMap::new();

        let mut worklist: VecDeque<lir::Block> = VecDeque::new();
        let mut pp_use: HashMap<String, HashSet<lir::Variable>> = HashMap::new();
        let mut pp_def: HashMap<String, HashSet<lir::Variable>> = HashMap::new();

        let addr_taken = prog.get_addr_taken(func_name);

        let mut solution = HashMap::new();
        for bb_label in &cfg.get_all_block_labels() {
            let block = cfg.get_block(bb_label).unwrap();
            for (idx, instr) in block.insts.iter().enumerate() {
                let pp = lir::ProgramPoint {
                    block: bb_label.clone(),
                    location: lir::Location::Instruction(idx),
                    instr: Some(instr.clone()),
                    term: None,
                };
                solution.insert(
                    pp.to_string(),
                    domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                );
                pp_use.insert(pp.to_string(), HashSet::new());
                pp_def.insert(pp.to_string(), HashSet::new());
            }
            let pp = lir::ProgramPoint {
                block: bb_label.clone(),
                location: lir::Location::Terminal,
                instr: None,
                term: Some(block.term.clone()),
            };
            solution.insert(
                pp.to_string(),
                domain::ProgramPoint::ProgramPointSet(HashSet::new()),
            );
            pp_use.insert(pp.to_string(), HashSet::new());
            pp_def.insert(pp.to_string(), HashSet::new());
        }

        let mut entry_store = store::ProgramPointStore::new();
        let mut bb2store = HashMap::new();

        // TODO: how to initialize entry_store
        prog.get_int_globals().iter().for_each(|global| {
            entry_store.set(
                global.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        prog.get_ptr_globals().iter().for_each(|global| {
            entry_store.set(
                global.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        prog.get_int_locals(func_name).iter().for_each(|local| {
            entry_store.set(
                local.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        prog.get_ptr_locals(func_name).iter().for_each(|local| {
            entry_store.set(
                local.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        prog.get_int_parameters(func_name).iter().for_each(|param| {
            entry_store.set(
                param.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        prog.get_ptr_parameters(func_name).iter().for_each(|param| {
            entry_store.set(
                param.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        addr_taken.iter().for_each(|var| {
            entry_store.set(
                var.clone(),
                // domain::ProgramPoint::ProgramPointSet(HashSet::new()),
                domain::ProgramPoint::Bottom,
            )
        });
        #[cfg(debug_assertions)]
        {
            println!("addr_taken: {:?}", addr_taken);
            println!("initialized entry_store:\n {}", entry_store.to_string());
        }
        worklist.push_back(cfg.get_entry().unwrap().clone());
        for bb_label in &cfg.get_all_block_labels() {
            bb2store.insert(bb_label.clone(), store::ProgramPointStore::new());
        }
        bb2store.insert("entry".to_string(), entry_store);

        Self {
            prog,
            bb2store,
            cfg,
            worklist,
            solution,
            pp_def,
            pp_use,
            addr_taken,
            executed: false,
        }
    }

    pub fn exe_pp(&mut self, pp: &lir::ProgramPoint) {
        // let block = self.cfg.get_block(&pp.block).unwrap();
        #[cfg(debug_assertions)]
        {
            println!();
            println!("executing pp: {}", pp.to_string());
        }
        let store = self.bb2store.get_mut(&pp.block).unwrap();
        self.pp_use.get_mut(&pp.to_string()).unwrap().clear(); // TODO: verify if this impacts
        self.pp_def.get_mut(&pp.to_string()).unwrap().clear(); // TODO: verify if this impacts
        match pp.location {
            lir::Location::Instruction(_) => {
                match pp.instr.as_ref().unwrap() {
                    // let instr = &block.insts[idx];
                    // match &block.insts[idx] {
                    Instruction::AddrOf { lhs, rhs: _ } => {
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                        // println!("added domain to store: {}", store.get(lhs).unwrap());
                    }
                    Instruction::Arith {
                        lhs,
                        aop: _,
                        op1,
                        op2,
                    } => {
                        if let lir::Operand::Var(var) = op1 {
                            // self.pp_use
                            //     .entry(pp.to_string())
                            //     .or_insert(HashSet::new())
                            //     .insert(var.clone());
                            self.pp_use
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .insert(var.clone());
                        }
                        if let lir::Operand::Var(var) = op2 {
                            // self.pp_use
                            //     .entry(pp.to_string())
                            //     .or_insert(HashSet::new())
                            //     .insert(var.clone());
                            self.pp_use
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .insert(var.clone());
                        }
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            // println!(
                            //     "joining store[{}] {} to solution[{}]",
                            //     var.name,
                            //     store.get(var).unwrap(),
                            //     pp.to_string()
                            // );
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
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
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            // println!(
                            //     "joining store[{}] {} to solution[{}]",
                            //     var.name,
                            //     store.get(var).unwrap(),
                            //     pp.to_string()
                            // );
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                        // println!("now the store is \n{}", store.to_string().blue());
                        // println!(
                        //     "now the pp_use[{}] is {:?}",
                        //     pp.to_string().green(),
                        //     self.pp_use.get(&pp.to_string()).unwrap()
                        // );
                    }
                    Instruction::Copy { lhs, op } => {
                        if let lir::Operand::Var(var) = op {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            // println!(
                            //     "joining store[{}] {} to solution[{}]",
                            //     var.name,
                            //     store.get(var).unwrap(),
                            //     pp.to_string()
                            // );
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Alloc { lhs, num, id: _ } => {
                        if let lir::Operand::Var(var) = num {
                            // self.pp_use
                            //     .entry(pp.to_string())
                            //     .or_insert(HashSet::new())
                            //     .insert(var.clone());
                            self.pp_use
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .insert(var.clone());
                        }
                        // self.pp_use
                        //     .entry(pp.to_string())
                        //     .or_insert(HashSet::new())
                        //     .insert(id.clone()); // TODO: ?????????
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());

                        // println!("now the store is {}", store.to_string().blue());
                        // println!(
                        //     "now the pp_use[{}] is {:?}",
                        //     pp.to_string().green(),
                        //     self.pp_use.get(&pp.to_string()).unwrap()
                        // );

                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
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
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Gfp { lhs, src, field: _ } => {
                        // get-field-pointer: `x = $gfp y foo` takes `y` (which is a pointer to a struct)
                        // and assigns to `x` the address of the `foo` field of the struct. the only way to
                        // access fields of a struct is via a pointer.
                        self.pp_use
                            .entry(pp.to_string())
                            .or_insert(HashSet::new())
                            .insert(src.clone());
                        // self.pp_use
                        //     .entry(pp.to_string())
                        //     .or_insert(HashSet::new())
                        //     .insert(field.clone()); // TODO: how to deal with field?
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Load { lhs, src } => {
                        // println!(
                        //     "executing load instruction {}: {:?}",
                        //     pp.to_string().green(),
                        //     pp.instr
                        // );
                        self.pp_use
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(src.clone());
                        for var in self.addr_taken.iter() {
                            if var.typ == lhs.typ {
                                self.pp_use
                                    .get_mut(&pp.to_string())
                                    .unwrap()
                                    .insert(var.clone());
                            }
                        }
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(lhs.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        store.set(
                            lhs.clone(),
                            domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                        );
                    }
                    Instruction::Store { dst, op } => {
                        // println!(
                        // "executing store instruction {}: {:?}",
                        // pp.to_string().green(),
                        // pp.instr
                        // );
                        self.pp_use
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(dst.clone());
                        if let lir::Operand::Var(var) = op {
                            self.pp_use
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .insert(var.clone());
                        }
                        match op {
                            lir::Operand::Var(var) => {
                                for addr_taken_var in self.addr_taken.iter() {
                                    if addr_taken_var.typ == var.typ {
                                        self.pp_def
                                            .get_mut(&pp.to_string())
                                            .unwrap()
                                            .insert(addr_taken_var.clone());
                                    }
                                }
                            }
                            lir::Operand::CInt(_) => {
                                for addr_taken_var in self.addr_taken.iter() {
                                    if addr_taken_var.typ == lir::Type::Int {
                                        self.pp_def
                                            .get_mut(&pp.to_string())
                                            .unwrap()
                                            .insert(addr_taken_var.clone());
                                    }
                                }
                            }
                        }
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }

                        for var in self.pp_def.get(&pp.to_string()).unwrap() {
                            // store.set(
                            //     var.clone(),
                            //     domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                            // );
                            store.set(
                                var.clone(),
                                store.get(var).unwrap().join(
                                    &domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                                ),
                            ) // TODO: consider using store.join
                        }
                        // println!("now the store is {}", store.to_string().blue());
                        // println!(
                        //     "now the pp_use[{}] is {:?}",
                        //     pp.to_string().green(),
                        //     self.pp_use.get(&pp.to_string()).unwrap()
                        // );
                    }
                    Instruction::CallExt {
                        lhs,
                        ext_callee: _,
                        args,
                    } => {
                        let mut sdef = HashSet::new(); // strongly defined
                        let mut wedf = HashSet::new(); // weakly defined
                        if let Some(lsh) = lhs {
                            sdef.insert(lsh.clone());
                        }
                        for global in self.prog.globals.iter() {
                            wedf.insert(global.clone()); // TODO: all types of globals???
                        }
                        let mut reached_types_via_globals = HashSet::new();
                        for global in self.prog.globals.iter() {
                            // reached_types_via_globals  = reached_types_via_globals.union(&global.typ.reachable_types(&self.prog)).cloned().collect();
                            reached_types_via_globals
                                .extend(global.typ.reachable_types(&self.prog));
                        }
                        for var in self.addr_taken.iter() {
                            if reached_types_via_globals.contains(&var.typ) {
                                wedf.insert(var.clone());
                            }
                        }
                        self.pp_use
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .extend(wedf.clone());
                        for arg in args {
                            if let lir::Operand::Var(var) = arg {
                                self.pp_use
                                    .get_mut(&pp.to_string())
                                    .unwrap()
                                    .insert(var.clone());
                            }
                        }
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .extend(sdef.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        for var in wedf.iter() {
                            store.set(
                                var.clone(),
                                store.get(var).unwrap().join(
                                    &domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                                ),
                            ) // TODO: 如何确定 store.get(var).unwrap() 一定存在？
                        }
                        if let Some(lsh) = lhs {
                            store.set(
                                lsh.clone(),
                                domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                            );
                        }
                    }
                }
            }

            // lir::Location::Terminal => match &block.term {
            lir::Location::Terminal => {
                match pp.term.as_ref().unwrap() {
                    Terminal::Jump(_) => {}
                    Terminal::Branch { cond, tt: _, ff: _ } => {
                        if let lir::Operand::Var(var) = cond {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                    }
                    Terminal::Ret(ret) => {
                        if let Some(lir::Operand::Var(var)) = ret {
                            self.pp_use
                                .entry(pp.to_string())
                                .or_insert(HashSet::new())
                                .insert(var.clone());
                        }
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                    }
                    Terminal::CallDirect {
                        lhs,
                        callee: _,
                        args,
                        next_bb: _,
                    } => {
                        let mut sdef = HashSet::new(); // strongly defined
                        let mut wdef = HashSet::new(); // weakly defined
                        if let Some(lsh) = lhs {
                            sdef.insert(lsh.clone());
                        }
                        for global in self.prog.globals.iter() {
                            wdef.insert(global.clone()); // TODO: all types of globals???
                        }
                        let mut reached_types_via_globals = HashSet::new();
                        for global in self.prog.globals.iter() {
                            // reached_types_via_globals  = reached_types_via_globals.union(&global.typ.reachable_types(&self.prog)).cloned().collect();
                            reached_types_via_globals
                                .extend(global.typ.reachable_types(&self.prog));
                        }
                        for var in self.addr_taken.iter() {
                            if reached_types_via_globals.contains(&var.typ) {
                                wdef.insert(var.clone());
                            }
                        }
                        self.pp_use
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .extend(wdef.clone());
                        for arg in args {
                            if let lir::Operand::Var(var) = arg {
                                self.pp_use
                                    .get_mut(&pp.to_string())
                                    .unwrap()
                                    .insert(var.clone());
                            }
                        }
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .extend(sdef.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        for var in wdef.iter() {
                            store.set(
                                var.clone(),
                                store.get(var).unwrap().join(
                                    &domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                                ),
                            )
                        }
                        if let Some(lsh) = lhs {
                            store.set(
                                lsh.clone(),
                                domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                            );
                        }
                    }
                    Terminal::CallIndirect {
                        lhs,
                        callee,
                        args,
                        next_bb: _,
                    } => {
                        let mut sdef = HashSet::new(); // strongly defined
                        let mut wdef = HashSet::new(); // weakly defined
                        if let Some(lsh) = lhs {
                            sdef.insert(lsh.clone());
                        }
                        for global in self.prog.globals.iter() {
                            wdef.insert(global.clone()); // TODO: all types of globals???
                        }
                        let mut reached_types_via_globals = HashSet::new();
                        for global in self.prog.globals.iter() {
                            // reached_types_via_globals  = reached_types_via_globals.union(&global.typ.reachable_types(&self.prog)).cloned().collect();
                            reached_types_via_globals
                                .extend(global.typ.reachable_types(&self.prog));
                        }
                        for var in self.addr_taken.iter() {
                            if reached_types_via_globals.contains(&var.typ) {
                                wdef.insert(var.clone());
                            }
                        }
                        self.pp_use
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .insert(callee.clone());
                        self.pp_use
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .extend(wdef.clone());
                        for arg in args {
                            if let lir::Operand::Var(var) = arg {
                                self.pp_use
                                    .get_mut(&pp.to_string())
                                    .unwrap()
                                    .insert(var.clone());
                            }
                        }
                        self.pp_def
                            .get_mut(&pp.to_string())
                            .unwrap()
                            .extend(sdef.clone());
                        for var in self.pp_use.get(&pp.to_string()).unwrap() {
                            self.solution
                                .get_mut(&pp.to_string())
                                .unwrap()
                                .join_in_place(store.get(var).unwrap());
                        }
                        for var in wdef.iter() {
                            store.set(
                                var.clone(),
                                store.get(var).unwrap().join(
                                    &domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                                ),
                            )
                        }
                        if let Some(lsh) = lhs {
                            store.set(
                                lsh.clone(),
                                domain::ProgramPoint::ProgramPointSet(hashset! {pp.clone()}),
                            );
                        }
                    }
                }
            }
        }
        #[cfg(debug_assertions)]
        {
            println!(
                "after executing:\n solution[{}] -> {:?}",
                pp.to_string(),
                self.solution.get(&pp.to_string()).unwrap().to_string()
            );
            println!("pp_use -> {:?}", self.pp_use.get(&pp.to_string()).unwrap());
            println!("pp_def -> {:?}", self.pp_def.get(&pp.to_string()).unwrap());
        }
    }
}

impl AbstractExecution for ReachingDefinitionAnalyzer {
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
        // There is an issue (use store_updated or store_joined ???) hard to deal with if using the following implementation:
        while !self.worklist.is_empty() {
            let block = self.worklist.pop_front().unwrap();
            self.exe_block(&block);
            visited.insert(block.id.clone(), visited.get(&block.id).unwrap() + 1);

            for succ in self.cfg.get_successors(&block) {
                let succ_store = self.bb2store.get(&succ.id).unwrap();
                let store_joined = succ_store.join(&self.bb2store.get(&block.id).unwrap());
                let mut new_store = store_joined.clone(); // it may be executed virtually
                                                          // TODO: try to update new_store???

                if &new_store != succ_store {
                    self.bb2store.insert(succ.id.clone(), store_joined);
                    if !self.worklist.contains(&succ) {
                        self.worklist.push_back(succ.clone());
                    }
                }
            }
        }
        */

        while !self.worklist.is_empty() {
            let block = self.worklist.pop_front().unwrap();
            let store_before = self.bb2store.get(&block.id).unwrap().clone();
            if &block != self.cfg.get_entry().unwrap() {
                // join ann predecessors' stores
                let mut store_joined = store::ProgramPointStore::new();
                for pred in self.cfg.get_predecessors(&block) {
                    store_joined = store_joined.join(&self.bb2store.get(&pred.id).unwrap());
                }
                self.bb2store.insert(block.id.clone(), store_joined);
            }
            self.exe_block(&block);
            let store_after = self.bb2store.get(&block.id).unwrap().clone();
            if store_before != store_after || visited.get(&block.id).unwrap() == &0 {
                // add all successors to worklist
                for succ in self.cfg.get_successors(&block) {
                    if !self.worklist.contains(&succ) {
                        // TODO: 这两行代码是否不需要？
                        self.worklist.push_back(succ.clone());
                    }
                }
            }
            visited.insert(block.id.clone(), visited.get(&block.id).unwrap() + 1);
        }

        // let mut worklist = Vec::new();
        // for (bb_label, _) in self.bb2store.iter() {
        //     worklist.push(bb_label.clone());
        // }
        // while !worklist.is_empty() {
        //     let bb_label = worklist.pop().unwrap();
        //     let store = self.bb2store.get(&bb_label).unwrap();
        //     let old_out = self.bb2out.get(&bb_label).unwrap().clone();
        //     let mut new_in = store.clone();
        //     let mut new_out = store.clone();
        //     for succ in self.reachable_successors.get(&bb_label).unwrap().iter() {
        //         let succ_in = self.bb2in.get(succ).unwrap();
        //         new_out = new_out.join(succ_in);
        //     }
        //     new_in = new_out.clone();
        //     new_in = new_in.minus_kill(&self.bb2kill.get(&bb_label).unwrap());
        //     new_in = new_in.join(&self.bb2gen.get(&bb_label).unwrap());
        //     if new_in != old_out {
        //         self.bb2in.insert(bb_label.clone(), new_in.clone());
        //         self.bb2out.insert(bb_label.clone(), new_out.clone());
        //         for pred in self.predecessors.get(&bb_label).unwrap().iter() {
        //             if !worklist.contains(pred) {
        //                 worklist.push(pred.clone());
        //             }
        //         }
        //     }
        // }
    }

    fn exe_block(&mut self, block: &lir::Block) {
        for (idx, instr) in block.insts.iter().enumerate() {
            let pp = lir::ProgramPoint {
                block: block.id.clone(),
                location: lir::Location::Instruction(idx),
                instr: Some(instr.clone()),
                term: None,
            };
            self.exe_pp(&pp);
        }
        let pp = lir::ProgramPoint {
            block: block.id.clone(),
            location: lir::Location::Terminal,
            instr: None,
            term: Some(block.term.clone()),
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

impl ControlDependenceAnalyzer {
    pub fn new(prog: lir::Program, func_name: &str) -> Self {
        let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
        let mut solution = HashMap::new();
        for bb_label in &cfg.get_all_block_labels() {
            solution.insert(bb_label.clone(), HashSet::new()); // TODO: correct?
        }
        Self {
            prog,
            cfg,
            solution,
            executed: false,
        }
    }

    pub fn execute(&mut self) {
        if self.executed {
            log::warn!("Already executed");
            return;
        }
        self.executed = true;
        /*

        def gene_frontiers(cfg: nx.DiGraph):
            frontiers = {node: set() for node in cfg.nodes}
            dominators = gene_dominators(cfg)
            for node, dom_nodes in dominators.items():
                strict_dom_nodes = dom_nodes - {node}
                for pred in cfg.predecessors(node):
                    for dom_pred in dominators[pred] - strict_dom_nodes:
                        frontiers[dom_pred] = frontiers[dom_pred].union({node})
            return frontiers

        */
        let dominators = self.cfg.get_dominators(false);
        for (label, doms) in &dominators {
            let strict_doms = doms
                .clone()
                .difference(&hashset! {label.clone()})
                .cloned()
                .collect();
            for pred in &self.cfg.get_predecessor_labels(label) {
                for dom_pred in dominators.get(pred).unwrap().difference(&strict_doms) {
                    self.solution
                        .get_mut(dom_pred)
                        .unwrap()
                        .insert(label.clone());
                }
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
