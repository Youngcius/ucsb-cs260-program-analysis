/*
Abstract domain, abstract semantics, and abstract execution.
*/

pub mod domain {
    use super::semantics::AbstractSemantics;
    use crate::lir;
    use std::ops::{Add, Div, Mul, Sub};

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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Number {
        NInfinity,
        Integer(i32),
        Infinity,
    }

    pub const NUM_ZERO: Number = Number::Integer(0);
    pub const NUM_ONE: Number = Number::Integer(1);
    pub const NUM_MINUS_ONE: Number = Number::Integer(-1);
    pub const TRUE_INTERVAL: Interval = Interval::Range(NUM_ONE, NUM_ONE);
    pub const FALSE_INTERVAL: Interval = Interval::Range(NUM_ZERO, NUM_ZERO);
    pub const UNDECIDED_INTERVAL: Interval = Interval::Range(NUM_ZERO, NUM_ONE);

    impl std::fmt::Display for Number {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Number::Integer(x) => write!(f, "{}", x),
                Number::Infinity => write!(f, "PosInf"),
                Number::NInfinity => write!(f, "NegInf"),
            }
        }
    }

    impl Add for Number {
        type Output = Number;

        fn add(self, other: Number) -> Number {
            match (self, other) {
                (Number::Integer(x), Number::Integer(y)) => Number::Integer(x + y),
                (Number::Infinity, Number::Integer(_)) => Number::Infinity,
                (Number::Integer(_), Number::Infinity) => Number::Infinity,
                (Number::NInfinity, Number::Integer(_)) => Number::NInfinity,
                (Number::Integer(_), Number::NInfinity) => Number::NInfinity,
                (Number::Infinity, Number::Infinity) => Number::Infinity,
                (Number::NInfinity, Number::NInfinity) => Number::NInfinity,
                _ => panic!("Addition of infinities is undefined"),
            }
        }
    }

    impl Sub for Number {
        type Output = Number;

        fn sub(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Number::Integer(x), Number::Integer(y)) => Number::Integer(x - y),
                (Number::Infinity, Number::Integer(_)) => Number::Infinity,
                (Number::Integer(_), Number::Infinity) => Number::NInfinity,
                (Number::NInfinity, Number::Integer(_)) => Number::NInfinity,
                (Number::Integer(_), Number::NInfinity) => Number::Infinity,
                (Number::Infinity, Number::NInfinity) => Number::Infinity,
                (Number::NInfinity, Number::Infinity) => Number::NInfinity,
                _ => panic!("Subtraction of infinities is undefined"),
            }
        }
    }

    impl Mul for Number {
        type Output = Number;

        fn mul(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (_, Number::Integer(0)) => Number::Integer(0),
                (Number::Integer(0), _) => Number::Integer(0),
                (Number::Integer(x), Number::Integer(y)) => Number::Integer(x * y),
                (Number::Infinity, Number::Integer(x)) => {
                    if x >= 0 {
                        Number::Infinity
                    } else {
                        Number::NInfinity
                    }
                }
                (Number::Integer(x), Number::Infinity) => {
                    if x >= 0 {
                        Number::Infinity
                    } else {
                        Number::NInfinity
                    }
                }
                (Number::NInfinity, Number::Integer(x)) => {
                    if x >= 0 {
                        Number::NInfinity
                    } else {
                        Number::Infinity
                    }
                }
                (Number::Integer(x), Number::NInfinity) => {
                    if x >= 0 {
                        Number::NInfinity
                    } else {
                        Number::Infinity
                    }
                }
                (Number::Infinity, Number::Infinity) => Number::Infinity,
                (Number::NInfinity, Number::NInfinity) => Number::Infinity,
                (Number::Infinity, Number::NInfinity) => Number::NInfinity,
                (Number::NInfinity, Number::Infinity) => Number::NInfinity,
            }
        }
    }

    impl Div for Number {
        type Output = Number;

        fn div(self, other: Number) -> Number {
            match (self, other) {
                (Number::Integer(x), Number::Integer(0)) => {
                    if x >= 0 {
                        Number::Infinity
                    } else {
                        Number::NInfinity
                    }
                }
                (Number::Infinity, Number::Integer(x)) => {
                    if x >= 0 {
                        Number::Infinity
                    } else {
                        Number::NInfinity
                    }
                }
                (Number::NInfinity, Number::Integer(x)) => {
                    if x >= 0 {
                        Number::NInfinity
                    } else {
                        Number::Infinity
                    }
                }
                (Number::Integer(_), Number::Infinity) => Number::Integer(0),
                (Number::Integer(_), Number::NInfinity) => Number::Integer(0),
                (Number::Integer(x), Number::Integer(y)) => Number::Integer(x / y),
                _ => panic!("Division of infinities is undefined"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Interval {
        Top,
        Bottom,
        Range(Number, Number),
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
        pub fn get_lower(&self) -> Option<Number> {
            match self {
                Self::Top => Some(Number::NInfinity),
                Self::Bottom => None,
                Self::Range(l, _) => Some(l.clone()),
            }
        }
        pub fn get_upper(&self) -> Option<Number> {
            match self {
                Self::Top => Some(Number::Infinity),
                Self::Bottom => None,
                Self::Range(_, u) => Some(u.clone()),
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
                    let l = if l1 <= l2 { *l1 } else { Number::NInfinity };
                    let u = if u1 >= u2 { *u1 } else { Number::Infinity };
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
                    if *l == Number::NInfinity && *u == Number::Infinity {
                        write!(f, "(NegInf, PosInf)")
                    } else if *l == Number::NInfinity {
                        write!(f, "(NegInf, {}]", u)
                    } else if *u == Number::Infinity {
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
                    let l = if l1 <= l2 { l1 } else { l2 };
                    let u = if u1 >= u2 { u1 } else { u2 };
                    Self::Range(*l, *u)
                }
            }
        }

        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self {
            match (self, other) {
                (Self::Bottom, _) => Self::Bottom,
                (_, Self::Bottom) => Self::Bottom,
                (Self::Top, Self::Top) => Self::Top,
                (Self::Top, Self::Range(l, u)) => match op {
                    lir::ArithOp::Add => Self::Top,
                    lir::ArithOp::Subtract => Self::Top,
                    lir::ArithOp::Multiply => {
                        if let Number::Integer(0) = l {
                            if let Number::Integer(0) = u {
                                Self::Range(Number::Integer(0), Number::Integer(0))
                            } else {
                                Self::Top
                            }
                        } else {
                            Self::Top
                        }
                    }
                    lir::ArithOp::Divide => {
                        if let Number::Integer(0) = l {
                            if let Number::Integer(0) = u {
                                Self::Bottom
                            } else {
                                Self::Top
                            }
                        } else {
                            Self::Top
                        }
                    }
                },
                (Self::Range(l, u), Self::Top) => match op {
                    lir::ArithOp::Add => Self::Top,
                    lir::ArithOp::Subtract => Self::Top,
                    lir::ArithOp::Multiply => {
                        if let Number::Integer(0) = l {
                            if let Number::Integer(0) = u {
                                Self::Range(Number::Integer(0), Number::Integer(0))
                            } else {
                                Self::Top
                            }
                        } else {
                            Self::Top
                        }
                    }
                    lir::ArithOp::Divide => {
                        if let Number::Integer(0) = l {
                            if let Number::Integer(0) = u {
                                Self::Range(Number::Integer(0), Number::Integer(0))
                            } else {
                                // I1 ÷ [-1, 1]
                                let mut v = vec![*l / NUM_MINUS_ONE, *l, *u / NUM_MINUS_ONE, *u];
                                v.sort();
                                Self::Range(v[0], v[3])
                            }
                        } else {
                            // I1 ÷ [-1, 1]
                            let mut v = vec![*l / NUM_MINUS_ONE, *l, *u / NUM_MINUS_ONE, *u];
                            v.sort();
                            Self::Range(v[0].clone(), v[3].clone())
                        }
                    }
                },
                (Self::Range(l1, u1), Self::Range(l2, u2)) => match op {
                    lir::ArithOp::Add => Self::Range(*l1 + *l2, *u1 + *u2),
                    lir::ArithOp::Subtract => Self::Range(*l1 - *u2, *u1 - *l2),
                    lir::ArithOp::Multiply => {
                        let mut v = vec![*l1 * *l2, *l1 * *u2, *u1 * *l2, *u1 * *u2];
                        v.sort();
                        Self::Range(v[0], v[3])
                    }
                    lir::ArithOp::Divide => {
                        if *l2 == Number::Integer(0) && *u2 == Number::Integer(0) {
                            Self::Bottom
                        } else if *l2 == Number::Integer(0) {
                            // I1 ÷ [1, I2.high]
                            let mut v = vec![*l1, *l1 / *u2, *u1, *u1 / *u2];
                            v.sort();
                            Self::Range(v[0], v[3])
                        } else if *u2 == Number::Integer(0) {
                            // I1 ÷ [I2.low, -1]
                            let mut v = vec![
                                *l1 / *l2,
                                *l1 / NUM_MINUS_ONE,
                                *u1 / *l2,
                                *u1 / NUM_MINUS_ONE,
                            ];
                            v.sort();
                            Self::Range(v[0], v[3])
                        } else if *l2 < Number::Integer(0) && *u2 > Number::Integer(0) {
                            // I1 ÷ [-1, 1]
                            let mut v = vec![*l1 / NUM_MINUS_ONE, *l1, *u1 / NUM_MINUS_ONE, *u1];
                            v.sort();
                            Self::Range(v[0], v[3])
                        } else {
                            let mut v = vec![*l1 / *l2, *l1 / *u2, *u1 / *l2, *u1 / *u2];
                            v.sort();
                            Self::Range(v[0], v[3])
                        }
                    }
                },
            }
        }

        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self {
            match (self, other) {
                (Self::Bottom, _) => Self::Bottom,
                (_, Self::Bottom) => Self::Bottom,
                (Self::Top, Self::Top) => UNDECIDED_INTERVAL,
                (Self::Top, Self::Range(_, _)) => UNDECIDED_INTERVAL,
                (Self::Range(_, _), Self::Top) => UNDECIDED_INTERVAL,
                (Self::Range(l1, u1), Self::Range(l2, u2)) => match op {
                    lir::RelaOp::Eq => {
                        if l1 == l2 && u1 == u2 && l1 == u1 {
                            TRUE_INTERVAL
                        } else if self.has_overlap(other) {
                            UNDECIDED_INTERVAL
                        } else {
                            FALSE_INTERVAL
                        }
                    }
                    lir::RelaOp::Neq => {
                        if l1 == l2 && u1 == u2 && l1 == u1 {
                            FALSE_INTERVAL
                        } else if self.has_overlap(other) {
                            UNDECIDED_INTERVAL
                        } else {
                            TRUE_INTERVAL
                        }
                    }
                    lir::RelaOp::Less => {
                        if u1 < l2 {
                            TRUE_INTERVAL
                        } else if l1 >= u2 {
                            FALSE_INTERVAL
                        } else {
                            UNDECIDED_INTERVAL
                        }
                    }
                    lir::RelaOp::LessEq => {
                        if u1 <= l2 {
                            TRUE_INTERVAL
                        } else if l1 > u2 {
                            FALSE_INTERVAL
                        } else {
                            UNDECIDED_INTERVAL
                        }
                    }
                    lir::RelaOp::Greater => {
                        if l1 > u2 {
                            TRUE_INTERVAL
                        } else if u1 <= l2 {
                            FALSE_INTERVAL
                        } else {
                            UNDECIDED_INTERVAL
                        }
                    }
                    lir::RelaOp::GreaterEq => {
                        if l1 >= u2 {
                            TRUE_INTERVAL
                        } else if u1 < l2 {
                            FALSE_INTERVAL
                        } else {
                            UNDECIDED_INTERVAL
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
    use super::domain::Number;
    use super::semantics::AbstractSemantics;
    use crate::cfg;
    use crate::lir;
    use crate::store;
    use crate::utils;
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
            let mut addrof_ints = prog.get_addrof_ints(func_name);
            // add global to addrof_ints
            for global in &global_ints {
                addrof_ints.push(global.clone());
            }
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

                    if visited.get(&block.id).unwrap() > &1 && visited.get(succ_label).unwrap() > &0
                    {
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
                                "\t store {} changed, pushing block {} to worklist",
                                succ_label, succ_label
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
                println!("Executing block {}", block.id);
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
                                    warn!("Copy: lhs and op type mismatch");
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
                warn!("Already executed");
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
                    let store_updated: store::Store<domain::Interval>; // 需要执行时真正加入的 store
                    let store_joined: store::Store<domain::Interval>;
                    let store_widened: store::Store<domain::Interval>;
                    let mut new_store: store::Store<domain::Interval>; // 判断是否需要加入 worklist
                    if visited.contains_key(succ_label) && loop_headers.contains(succ_label) {
                        // println!("{} is a loop header", succ_label);
                        // println!("{} \n▽\n {}", succ_store.to_string(), self.bb2store.get(&block.id).unwrap().to_string());
                        store_widened = succ_store.widen(&self.bb2store.get(&block.id).unwrap());
                        // println!("After widening: \n{}", store_widened.to_string());
                        store_updated = store_widened.clone();
                        new_store = store_widened.clone();
                    } else {
                        store_joined = succ_store.join(&self.bb2store.get(&block.id).unwrap());
                        store_updated = succ_store.update(&self.bb2store.get(&block.id).unwrap());
                        new_store = store_joined.clone(); // it may be executed virtually
                    }

                    // let mut new_store = store_joined.clone(); // it may be executed virtually

                    // if visited.get(&block.id).unwrap() > &1 && visited.get(succ_label).unwrap() > &0
                    // {
                    //     // it is a block in a loop
                    //     #[cfg(debug_assertions)]
                    //     {
                    //         println!("\n{} is a block in a loop\n", succ_label);
                    //     }
                    //     let mut analyzer_duplicate = self.clone();
                    //     analyzer_duplicate
                    //         .bb2store
                    //         .insert(succ_label.clone(), store_updated.clone());
                    //     // println!("______ in duplicate _____");
                    //     analyzer_duplicate.exe_block(&succ);
                    //     // println!("------ duplicate ------");
                    //     new_store = analyzer_duplicate.bb2store.get(succ_label).unwrap().clone();
                    // }

                    if &new_store != succ_store {
                        #[cfg(debug_assertions)]
                        {
                            println!(
                                "\t store {} to be changed (after executing {}), pushed to worklist",
                                succ_label, block.id
                            );
                        }
                        // if self.cfg.is_edge_in_cycle(&block.id, succ_label){
                        self.bb2store.insert(succ_label.clone(), new_store);
                        // } else {
                        // self.bb2store.insert(succ_label.clone(), store_updated);
                        // }
                        self.worklist.push_back(succ.clone());
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
                                    warn!("Copy: lhs and op type mismatch");
                                    res_val = domain::Interval::Top;
                                }
                            }
                            lir::Operand::CInt(c) => {
                                res_val = domain::Interval::Range(
                                    Number::Integer(*c),
                                    Number::Integer(*c),
                                );
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
                                let op2_val = domain::Interval::Range(
                                    Number::Integer(*c),
                                    Number::Integer(*c),
                                );
                                res_val = op1_val.arith(&op2_val, aop);
                            } else {
                                res_val = domain::Interval::Top;
                            }
                        }
                        (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
                            if let lir::Type::Int = var.typ {
                                let op1_val = domain::Interval::Range(
                                    Number::Integer(*c),
                                    Number::Integer(*c),
                                );
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
                lir::Instruction::Store { dst, op } => {
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
                                } else {
                                    res_val = domain::UNDECIDED_INTERVAL;
                                }
                            }
                            (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
                                let op1_val = domain::Interval::Range(
                                    Number::Integer(*c1),
                                    Number::Integer(*c1),
                                );
                                let op2_val = domain::Interval::Range(
                                    Number::Integer(*c2),
                                    Number::Integer(*c2),
                                );
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
                                        self.reachable_successors.insert(
                                            bb_label.to_string(),
                                            vec![tt.clone(), ff.clone()],
                                        );
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
}
