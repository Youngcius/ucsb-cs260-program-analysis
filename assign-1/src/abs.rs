/*
Abstract domain, abstract semantics, and abstract execution.
*/

pub mod domain {
    use super::semantics::AbstractSemantics;
    use crate::lir;

    #[derive(Debug, Clone)]
    pub enum DomainType {
        Constant,
        Interval,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Constant {
        Top,
        Bottom,
        CInt(i32),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Interval {
        Top,
        Bottom,
        Range(i32, i32),
    }

    impl std::fmt::Display for Constant {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Bottom => write!(f, "⊥"),
                Self::Top => write!(f, "Top"),
                Self::CInt(c) => write!(f, "{}", c),
            }
        }
    }

    impl Interval {
        pub fn get_lower(&self) -> String {
            match self {
                Self::Top => "NegInf".to_string(),
                Self::Bottom => "None".to_string(),
                Self::Range(l, _) => {
                    if *l == i32::MIN {
                        "NegInf".to_string()
                    } else {
                        l.to_string()
                    }
                }
            }
        }
        pub fn get_upper(&self) -> String {
            match self {
                Self::Top => "PosInf".to_string(),
                Self::Bottom => "None".to_string(),
                Self::Range(_, u) => {
                    if *u == i32::MAX {
                        "PosInf".to_string()
                    } else {
                        u.to_string()
                    }
                }
            }
        }
        pub fn widen(&self, other: &Self) -> Self {
            match (self, other) {
                (Self::Bottom, _) => other.clone(),
                (_, Self::Bottom) => self.clone(),
                (Self::Top, Self::Top) => Self::Top,
                (Self::Top, Self::Range(_, _)) => Self::Top,
                (Self::Range(_, _), Self::Top) => Self::Top,
                (Self::Range(l1, u1), Self::Range(l2, u2)) => {
                    let l = if l1 < l2 { *l1 } else { i32::MIN };
                    let u = if u1 > u2 { *u1 } else { i32::MAX };
                    Self::Range(l, u)
                }
            }
        }
        pub fn has_overlap(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Bottom, _) => false,
                (_, Self::Bottom) => false,
                (Self::Top, _) => true,
                (_, Self::Top) => true,
                (Self::Range(l1, u1), Self::Range(l2, u2)) => {
                    if l1 <= l2 && l2 <= u1 {
                        true
                    } else if l1 <= u2 && u2 <= u1 {
                        true
                    } else if l2 <= l1 && l1 <= u2 {
                        true
                    } else if l2 <= u1 && u1 <= u2 {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    impl std::fmt::Display for Interval {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Bottom => write!(f, "⊥"),
                Self::Top => write!(f, "(NegInf, PosInf)"),
                Self::Range(l, u) => {
                    if *l == i32::MIN && *u == i32::MAX {
                        write!(f, "(NegInf, PosInf)")
                    } else if *l == i32::MIN {
                        write!(f, "(NegInf, {}]", u)
                    } else if *u == i32::MAX {
                        write!(f, "[{}, PosInf)", l)
                    } else {
                        write!(f, "[{}, {}]", l, u)
                    }
                }
            }
        }
    }

    impl AbstractSemantics for Constant {
        fn is_bottom(&self) -> bool {
            match self {
                Self::Bottom => true,
                _ => false,
            }
        }
        fn is_top(&self) -> bool {
            match self {
                Self::Top => true,
                _ => false,
            }
        }
        fn join(&self, other: &Self) -> Self {
            match (self, other) {
                (Self::Bottom, _) => other.clone(),
                (_, Self::Bottom) => self.clone(),
                (Self::CInt(c1), Self::CInt(c2)) => {
                    if c1 == c2 {
                        self.clone()
                    } else {
                        Self::Top
                    }
                }
                _ => Self::Top,
            }
        }
        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self {
            match (self, other) {
                (Self::Bottom, _) => Self::Bottom,
                (_, Self::Bottom) => Self::Bottom,
                (Self::Top, Self::Top) => Self::Top,
                (Self::CInt(c), Self::Top) => match op {
                    lir::ArithOp::Add => Self::Top,
                    lir::ArithOp::Subtract => Self::Top,
                    lir::ArithOp::Multiply => {
                        if *c == 0 {
                            Self::CInt(0)
                        } else {
                            Self::Top
                        }
                    }
                    lir::ArithOp::Divide => {
                        if *c == 0 {
                            Self::CInt(0)
                        } else {
                            Self::Top
                        }
                    }
                },
                (Self::Top, Self::CInt(c)) => match op {
                    lir::ArithOp::Add => Self::Top,
                    lir::ArithOp::Subtract => Self::Top,
                    lir::ArithOp::Multiply => {
                        if *c == 0 {
                            Self::CInt(0)
                        } else {
                            Self::Top
                        }
                    }
                    lir::ArithOp::Divide => {
                        if *c == 0 {
                            Self::Bottom
                        } else {
                            Self::Top
                        }
                    }
                },
                (Self::CInt(c1), Self::CInt(c2)) => {
                    let c = match op {
                        lir::ArithOp::Add => c1 + c2,
                        lir::ArithOp::Subtract => c1 - c2,
                        lir::ArithOp::Multiply => c1 * c2,
                        lir::ArithOp::Divide => {
                            if *c2 == 0 {
                                return Self::Bottom;
                            }
                            c1 / c2
                        }
                    };
                    Self::CInt(c)
                }
            }
        }
        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self {
            match (self, other) {
                (Self::Bottom, _) => Self::Bottom,
                (_, Self::Bottom) => Self::Bottom,
                (Self::Top, Self::Top) => Self::Top,
                (Self::Top, Self::CInt(_)) => Self::Top,
                (Self::CInt(_), Self::Top) => Self::Top,
                (Self::CInt(c1), Self::CInt(c2)) => {
                    let c = match op {
                        lir::RelaOp::Eq => c1 == c2,
                        lir::RelaOp::Neq => c1 != c2,
                        lir::RelaOp::Less => c1 < c2,
                        lir::RelaOp::LessEq => c1 <= c2,
                        lir::RelaOp::Greater => c1 > c2,
                        lir::RelaOp::GreaterEq => c1 >= c2,
                    };
                    Self::CInt(c as i32)
                }
            }
        }
    }

    impl AbstractSemantics for Interval {
        fn is_bottom(&self) -> bool {
            match self {
                Self::Bottom => true,
                _ => false,
            }
        }
        fn is_top(&self) -> bool {
            match self {
                Self::Top => true,
                _ => false,
            }
        }
        fn join(&self, other: &Self) -> Self {
            match (self, other) {
                (Self::Bottom, _) => other.clone(),
                (_, Self::Bottom) => self.clone(),
                (Self::Top, Self::Top) => Self::Top,
                (Self::Top, Self::Range(_, _)) => Self::Top,
                (Self::Range(_, _), Self::Top) => Self::Top,
                (Self::Range(l1, u1), Self::Range(l2, u2)) => {
                    let l = if l1 < l2 { l1 } else { l2 };
                    let u = if u1 > u2 { u1 } else { u2 };
                    Self::Range(*l, *u)
                }
            }
        }

        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self {
            match (self, other) {
                (Self::Bottom, _) => other.clone(),
                (_, Self::Bottom) => self.clone(),
                (Self::Top, Self::Top) => Self::Top,
                (Self::Top, Self::Range(_, _)) => Self::Top,
                (Self::Range(_, _), Self::Top) => Self::Top,
                (Self::Range(l1, u1), Self::Range(l2, u2)) => match op {
                    lir::ArithOp::Add => Self::Range(l1 + l2, u1 + u2),
                    lir::ArithOp::Subtract => Self::Range(l1 - u2, u1 - l2),
                    lir::ArithOp::Multiply => {
                        let mut v = vec![l1 * l2, l1 * u2, u1 * l2, u1 * u2];
                        v.sort();
                        Self::Range(v[0], v[3])
                    }
                    lir::ArithOp::Divide => {
                        if *l2 == 0 && *u2 == 0 {
                            Self::Bottom
                        } else if *l2 == 0 {
                            // I1 ÷ [1, I2.high]
                            let mut v: Vec<i32> = vec![*l1, l1 / u2, *u1, u1 / u2];
                            v.sort();
                            Self::Range(v[0], v[3])
                        } else if *u2 == 0 {
                            // I1 ÷ [I2.low, -1]
                            let mut v = vec![l1 / l2, -l1, u1 / l2, -u1];
                            v.sort();
                            Self::Range(v[0], v[3])
                        } else if *l2 < 0 && *u2 > 0 {
                            // I1 ÷ [-1, 1]
                            let mut v: Vec<i32> = vec![-l1, *l1, -u1, *u1];
                            v.sort();
                            Self::Range(v[0], v[3])
                        } else {
                            let mut v: Vec<i32> = vec![l1 / l2, l1 / u2, u1 / l2, u1 / u2];
                            v.sort();
                            Self::Range(v[0], v[3])
                        }
                    }
                },
            }
        }

        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self {
            let true_interval = Self::Range(1, 1);
            let false_interval = Self::Range(0, 0);
            let undecided_interval = Self::Range(0, 1);
            match (self, other) {
                (Self::Bottom, _) => Self::Bottom,
                (_, Self::Bottom) => Self::Bottom,
                (Self::Top, Self::Top) => undecided_interval,
                (Self::Top, Self::Range(_, _)) => undecided_interval,
                (Self::Range(_, _), Self::Top) => undecided_interval,
                (Self::Range(l1, u1), Self::Range(l2, u2)) => match op {
                    lir::RelaOp::Eq => {
                        if l1 == l2 && u1 == u2 && l1 == u1 {
                            true_interval
                        } else if self.has_overlap(other) {
                            undecided_interval
                        } else {
                            false_interval
                        }
                    }
                    lir::RelaOp::Neq => {
                        if l1 == l2 && u1 == u2 && l1 == u1 {
                            false_interval
                        } else if self.has_overlap(other) {
                            undecided_interval
                        } else {
                            true_interval
                        }
                    }
                    lir::RelaOp::Less => {
                        if self.has_overlap(other) {
                            undecided_interval
                        } else {
                            if u1 < l2 {
                                true_interval
                            } else {
                                false_interval
                            }
                        }
                    }
                    lir::RelaOp::LessEq => {
                        if self.has_overlap(other) {
                            if u1 == l2 {
                                true_interval
                            } else if l1 == u2 {
                                false_interval
                            } else {
                                undecided_interval
                            }
                        } else {
                            if u1 <= l2 {
                                true_interval
                            } else {
                                false_interval
                            }
                        }
                    }
                    lir::RelaOp::Greater => {
                        if self.has_overlap(other) {
                            undecided_interval
                        } else {
                            if l1 > u2 {
                                true_interval
                            } else {
                                false_interval
                            }
                        }
                    }
                    lir::RelaOp::GreaterEq => {
                        if self.has_overlap(other) {
                            if l1 == u2 {
                                true_interval
                            } else if u1 == l2 {
                                false_interval
                            } else {
                                undecided_interval
                            }
                        } else {
                            if l1 >= u2 {
                                true_interval
                            } else {
                                false_interval
                            }
                        }
                    }
                },
            }
        }
    }
}

pub mod semantics {
    use crate::lir;

    pub trait AbstractSemantics {
        fn is_bottom(&self) -> bool;
        fn is_top(&self) -> bool;
        fn join(&self, other: &Self) -> Self;
        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self;
        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self;
    }
}

pub mod execution {

    use super::domain;
    use super::semantics::AbstractSemantics;
    use crate::cfg;
    use crate::lir;
    use crate::store;
    use crate::utils;
    use colored::Colorize;
    use log::warn;
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
            // let addrof_ints = prog.get_int_globals();
            let mut addrof_ints = prog.get_addrof_ints(func_name);
            // add global to addrof_ints
            for global in &global_ints {
                // println!("adding {} to addrof_ints", global.name);
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
            // for addr_int in &addrof_ints {
            //     // println!("adding {} to entry_store (TOP)", addr_int.name);
            //     entry_store.set(addr_int.clone(), domain::Constant::Top);
            // }
            for param in &param_ints {
                // println!("adding {} to entry_store (TOP)", param.name);
                entry_store.set(param.clone(), domain::Constant::Top);
            }

            #[cfg(debug_assertions)]
            {
                println!("ENTRY_STORE:");
                let mut var_names = entry_store.get_var_names();
                var_names.sort();
                for var_name in &var_names {
                    let abs_val = entry_store.get_by_name(var_name).unwrap();
                    println!("{} -> {}\n", var_name, abs_val);
                }
                println!("---------------------------------");
            }

            worklist.push_back(cfg.get_entry().unwrap().clone());
            #[cfg(debug_assertions)]
            {
                println!("worklist: {:?}", worklist);
                println!("entry_store:");
                println!("{}", &entry_store);
            }
            for bb_label in &cfg.get_all_block_labels() {
                bb2store.insert(bb_label.clone(), store::ConstantStore::new());
            }
            // bb2store.insert("dummy_entry".to_string(), entry_store);

            bb2store.insert("entry".to_string(), entry_store);

            #[cfg(debug_assertions)]
            {
                println!(
                    "bb2store.len: {}, {:?}",
                    bb2store.len(),
                    bb2store.keys().collect::<Vec<&String>>()
                );
                print!("global_ints: ");
                for var in global_ints.iter() {
                    print!("{}, ", var.name.green());
                }
                println!();
            }
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
            // Initialized the interval analyzer
            let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
            let reachable_successors: HashMap<String, Vec<String>> = HashMap::new();
            let mut worklist: VecDeque<lir::Block> = VecDeque::new();
            let mut bb2store: HashMap<String, store::IntervalStore> = HashMap::new();
            let mut entry_store = store::IntervalStore::new();
            // set content of entry_store
            let global_ints = prog.get_int_globals();
            let param_ints = prog.get_int_parameters(func_name);
            let local_ints = prog.get_int_locals(func_name);
            let addrof_ints = prog.get_int_globals();
            for global in &global_ints {
                entry_store.set(global.clone(), domain::Interval::Top);
            }
            for param in &param_ints {
                entry_store.set(param.clone(), domain::Interval::Top);
            }
            for local in &local_ints {
                entry_store.set(local.clone(), domain::Interval::Bottom);
            }
            worklist.push_back(cfg.get_entry().unwrap().clone());
            #[cfg(debug_assertions)]
            {
                println!("worklist: {:?}", worklist);
                println!("entry_store:");
                println!("{}", entry_store);
            }
            for bb_label in &cfg.get_all_block_labels() {
                bb2store.insert(bb_label.clone(), store::IntervalStore::new());
            }
            bb2store.insert("entry".to_string(), entry_store);
            #[cfg(debug_assertions)]
            {
                println!(
                    "bb2store.len: {}, {:?}",
                    bb2store.len(),
                    cfg.get_all_block_labels()
                );
                print!("global_ints: ");
                for var in global_ints.iter() {
                    print!("{}, ", var.name.green());
                }
                println!();
            }
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
                warn!("Already executed");
                return;
            }
            let mut visited: HashMap<String, u32> = HashMap::new(); // <bb_label, count>
            for bb_label in self.cfg.get_all_block_labels() {
                visited.insert(bb_label.clone(), 0);
            }
            self.executed = true;
            while !self.worklist.is_empty() {
                let block = self.worklist.pop_front().unwrap();

                #[cfg(debug_assertions)]
                {
                    println!("Pop block id={} from worklist", block.id.blue());
                }

                self.exe_block(&block);

                visited.insert(block.id.clone(), visited.get(&block.id).unwrap() + 1);

                #[cfg(debug_assertions)]
                {
                    println!(
                        "reachable_successors of {}: {:?}",
                        block.id.blue(),
                        self.reachable_successors
                    );
                }

                for succ_label in self.reachable_successors.get(&block.id).unwrap() {
                    #[cfg(debug_assertions)]
                    {
                        println!("successor label of {}: {}", block.id, succ_label);
                        println!(
                            "Joining store {} (just executed) --> store {}",
                            block.id.green(),
                            succ_label.green()
                        );
                    }

                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "Joining store {} (just executed) --> store {}",
                            block.id.green(),
                            succ_label.green()
                        );
                        println!(
                            "{}",
                            self.bb2store.get(&block.id).unwrap().to_string().blue()
                        );
                        println!();
                        println!(
                            "{}",
                            self.bb2store.get(succ_label).unwrap().to_string().blue()
                        );
                    }

                    let succ = self.cfg.get_block(&succ_label).unwrap().clone();
                    let succ_store = self.bb2store.get(succ_label).unwrap(); // succ_store before joining and executing
                    let store_joined = succ_store.join(&self.bb2store.get(&block.id).unwrap());
                    let mut new_store = store_joined.clone(); // it may be executed virtually

                    // self.exe_block(&succ);
                    if visited.get(&block.id).unwrap() > &1 && visited.get(succ_label).unwrap() > &0
                    {
                        // it is a block in a loop
                        #[cfg(debug_assertions)]
                        {
                            println!("\n{} is a block in a loop\n", succ_label.red());
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
                                "\t store {} changed, pushing block {} to worklist",
                                succ_label.green(),
                                succ_label.green()
                            );
                        }
                        self.bb2store.insert(succ_label.clone(), store_joined);
                        self.worklist.push_back(succ.clone());
                    }
                }
            }
        }

        fn exe_block(&mut self, block: &lir::Block) {
            #[cfg(debug_assertions)]
            {
                println!("Executing block {}", block.id.blue());
            }
            for instr in &block.insts {
                self.exe_instr(instr, &block.id);
            }
            self.exe_term(&block.term, &block.id);
            #[cfg(debug_assertions)]
            {
                println!()
            }
        }

        fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str) {
            /*
            execute an instruction on the store
            current support: no-function no-pointer
            op: Operand::CInt(i32), or Operand::Var { name: String, typ: Type::Int, scope: ...}
            */
            #[cfg(debug_assertions)]
            {
                println!("executing instruction: {:?}", instr);
            }
            let store = self.bb2store.get_mut(bb_label).unwrap();
            match instr {
                lir::Instruction::AddrOf { lhs, rhs } => {
                    // {"AddrOf": {"lhs": "xxx", "rhs": "xxx"}}
                    if let lir::Type::Int = rhs.typ {
                        // self.addrof_ints.push(rhs.clone());
                        assert!(self.addrof_ints.contains(rhs));
                        // 直到这时才设置其 abstract value 为 Top???
                        // store.set(rhs.clone(), domain::Constant::Top);
                        #[cfg(debug_assertions)]
                        {
                            println!("added {} to addrof_ints", rhs.name.red());
                        }
                    }
                }
                lir::Instruction::Alloc { lhs, num, id } => {
                    // {"Alloc": {"lhs": "xxx", "num": "xxx", "id": "xxx"}}
                    // let num_val = store.get(num).unwrap();
                    // let id_val = store.get(id).unwrap();
                    // store.set(lhs.clone(), id_val.clone());
                }
                lir::Instruction::Copy { lhs, op } => {
                    // {"Copy": {"lhs": "xxx", "op": "xxx"}}
                    if let lir::Type::Int = lhs.typ {
                        let res_val: domain::Constant;
                        match op {
                            lir::Operand::Var(var) => {
                                if let lir::Type::Int = var.typ {
                                    res_val = store.get(var).unwrap().clone();
                                    #[cfg(debug_assertions)]
                                    {
                                        println!(
                                            "[COPY] lhs: {}, op: {:?}, res_val: {} (block: {})",
                                            lhs.name,
                                            var.name,
                                            res_val.to_string().green(),
                                            bb_label
                                        );
                                    }
                                    store.set(lhs.clone(), store.get(var).unwrap().clone());
                                } else {
                                    warn!("Copy: lhs and op type mismatch");
                                    res_val = domain::Constant::Top;
                                }
                            }
                            lir::Operand::CInt(c) => {
                                res_val = domain::Constant::CInt(*c);
                                // store.set(lhs.clone(), domain::Constant::CInt(*c));
                                #[cfg(debug_assertions)]
                                {
                                    println!(
                                        "[COPY] lhs: {}, op: {:?}, res_val: {} (block: {})",
                                        lhs.name,
                                        c,
                                        res_val.to_string().green(),
                                        bb_label
                                    );
                                }
                            }
                        }
                        store.set(lhs.clone(), res_val);
                        // println!("after COPY:\n{}", store.to_string().red());
                    }
                }
                lir::Instruction::Gep { lhs, src, idx } => {
                    // {"Gep": {"lhs": "xxx", "src": "xxx", "idx": "xxx"}}
                    // let src_val = store.get(src).unwrap();
                    // let idx_val = store.get(idx).unwrap();
                    // store.set(lhs.clone(), src_val.clone());
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
                            // let op1_val = store.get(var1).unwrap();
                            // let op2_val = store.get(var2).unwrap();
                            // res_val = op1_val.arith(op2_val, aop);
                        }
                        (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = store.get(var).unwrap();
                                let op2_val = domain::Constant::CInt(*c);
                                res_val = op1_val.arith(&op2_val, aop);
                            } else {
                                res_val = domain::Constant::Top;
                            }
                            // let op1_val = store.get(var).unwrap();
                            // let op2_val = domain::Constant::CInt(*c);
                            // res_val = op1_val.arith(&op2_val, aop);
                        }
                        (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = domain::Constant::CInt(*c);
                                let op2_val = store.get(var).unwrap();
                                res_val = op1_val.arith(op2_val, aop);
                            } else {
                                res_val = domain::Constant::Top;
                            }
                            // let op1_val = domain::Constant::CInt(*c);
                            // let op2_val = store.get(var).unwrap();
                            // res_val = op1_val.arith(op2_val, aop);
                        }
                        (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
                            let op1_val = domain::Constant::CInt(*c1);
                            let op2_val = domain::Constant::CInt(*c2);
                            res_val = op1_val.arith(&op2_val, aop);
                        }
                    }
                    store.set(lhs.clone(), res_val);
                }
                lir::Instruction::Load { lhs, src } => {
                    // {"Load": {"lhs": "xxx", "src": "xxx"}
                    if let lir::Type::Int = lhs.typ {
                        store.set(lhs.clone(), domain::Constant::Top);
                    }
                }
                lir::Instruction::Store { dst, op } => {
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
                                println!("Now new_store: {}", new_store.to_string().blue());
                            }
                            #[cfg(debug_assertions)]
                            {
                                println!("In Store instruction, joining store with new_store");
                                println!("Before joining:");
                                println!("{}", store.to_string().green());
                            }

                            *store = store.join(&new_store);
                            #[cfg(debug_assertions)]
                            {
                                println!("After joining:");
                                println!("{}", store.to_string().red());
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
                                    println!("Now new_store: {}", new_store.to_string().blue());
                                }
                                #[cfg(debug_assertions)]
                                {
                                    println!("In Store instruction, joining store with new_store");
                                    println!("Before joining:");
                                    println!("{}", store.to_string().green());
                                }
                                *store = store.join(&new_store);
                                #[cfg(debug_assertions)]
                                {
                                    println!("After joining:");
                                    println!("{}", store.to_string().red());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                lir::Instruction::Gfp { lhs, src, field } => {
                    // {"Gfp": {"lhs": "xxx", "src": "xxx", "field": "xxx"}}
                    // let src_val = store.get(src).unwrap();
                    // let field_val = store.get(field).unwrap();
                    // store.set(lhs.clone(), src_val.clone());
                }
                lir::Instruction::Cmp { lhs, rop, op1, op2 } => {
                    // {"Cmp": {"lhs": "xxx", "rop": "xxx", "op1": "xxx", "op2": "xxx"}}
                    // println!(
                    //     "[CMP] lhs: {}, op1: {:?}, op2: {}, res_val: {} (block: {})",
                    //     lhs.name,
                    //     var.name,
                    //     res_val.to_string().green(),
                    //     bb_label
                    // );
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
                                        #[cfg(debug_assertions)]
                                        {
                                            println!("\t[CMP] comparing two int-type variables: ({} -> {}), ({} -> {})", var1.name, op1_val.to_string().green(), var2.name, op2_val.to_string().green());
                                        }
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
                    ext_callee,
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
                    callee,
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
                    callee,
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
                warn!("Already executed");
                return;
            }
            self.executed = true;
            while !self.worklist.is_empty() {
                let block = self.worklist.pop_front().unwrap();
                self.exe_block(&block);

                self.cfg
                    .get_successor_labels(&block.id)
                    .iter()
                    .for_each(|succ_label| {
                        let succ = self.cfg.get_block(succ_label).unwrap();
                        let succ_store = self.bb2store.get(succ_label).unwrap();
                        let new_store = succ_store.join(&self.bb2store.get(&block.id).unwrap());
                        if new_store != succ_store.clone() {
                            self.bb2store.insert(succ_label.clone(), new_store.clone());
                            self.worklist.push_back(succ.clone());
                        }
                    });
            }

            // TODO: 找到 loop header, 执行 widening
        }
        fn exe_block(&mut self, block: &lir::Block) {
            for instr in &block.insts {
                self.exe_instr(instr, &block.id);
            }
            self.exe_term(&block.term, &block.id);
        }
        fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str) {
            panic!("Not implemented")
        }
        fn exe_term(&mut self, term: &lir::Terminal, bb_label: &str) {
            let store = self.bb2store.get_mut(bb_label).unwrap();
            match term {
                lir::Terminal::CallDirect {
                    lhs,
                    callee,
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
                }
                lir::Terminal::CallIndirect {
                    lhs,
                    callee,
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
                }
                _ => {}
            }
        }
    }

    pub trait AbstractExecution {
        fn mfp(&mut self);
        fn exe_block(&mut self, block: &lir::Block);
        fn exe_instr(&mut self, instr: &lir::Instruction, bb_label: &str);
        fn exe_term(&mut self, term: &lir::Terminal, bb_label: &str);
    }
}

#[cfg(test)]
mod test {
    use super::domain::Interval;

    #[test]
    fn test_interval_output() {
        let bottom = Interval::Bottom;
        let top = Interval::Top;
        let range = Interval::Range(1, 2);

        println!(
            "lower of bottom: {}, upper of bottom: {}",
            bottom.get_lower(),
            bottom.get_upper()
        );
        println!(
            "lower of top: {}, upper of top: {}",
            top.get_lower(),
            top.get_upper()
        );
        println!(
            "lower of range: {}, upper of range: {}",
            range.get_lower(),
            range.get_upper()
        );

        assert_eq!(bottom.to_string(), "⊥");
        assert_eq!(top.to_string(), "(NegInf, PosInf)");
        assert_eq!(range.to_string(), "[1, 2]");
    }
}
