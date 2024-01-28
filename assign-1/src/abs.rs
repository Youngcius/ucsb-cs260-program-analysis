/*
Abstract domain, abstract semantics, and abstract execution.
*/

pub mod domain {
    use super::semantics::AbstractSemantics;
    use crate::lir;

    #[derive(Debug)]
    pub enum DomainType {
        Constant,
        Interval,
    }

    #[derive(Debug, Clone)]
    pub enum Constant {
        Top,
        Bottom,
        CInt(i32),
    }

    #[derive(Debug, Clone)]
    pub enum Interval {
        Top,
        Bottom,
        Range(i32, i32),
    }

    impl Interval {
        pub fn get_lower(&self) -> Option<i32> {
            match self {
                Self::Top => Some(i32::MIN),
                Self::Bottom => None,
                Self::Range(l, _) => Some(*l),
            }
        }
        pub fn get_upper(&self) -> Option<i32> {
            match self {
                Self::Top => Some(i32::MAX),
                Self::Bottom => None,
                Self::Range(_, u) => Some(*u),
            }
        }
    }

    impl AbstractSemantics for Constant {
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
                (Self::Top, Self::Top) => Self::Top,
                (Self::Top, Self::CInt(_)) => Self::Top,
                (Self::CInt(_), Self::Top) => Self::Top,
                (Self::Bottom, _) => Self::Bottom,
                (_, Self::Bottom) => Self::Bottom,
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
        fn join(&self, other: &Self) -> Self {
            // TODO
            panic!("TODO")
        }
        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self {
            // TODO
            panic!("TODO")
        }
        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self {
            // TODO
            panic!("TODO")
        }
    }
}

pub mod semantics {
    use crate::lir;

    pub trait AbstractSemantics {
        fn join(&self, other: &Self) -> Self;
        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self;
        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self;
    }
}

pub mod execution {

    use std::collections::HashMap;

    use super::domain;
    use crate::cfg;
    use crate::lir;
    use crate::store;

    // fn mfp(prog: &lir::Program) {
    //     // MFP (Meet For All Paths) worklist algorithm
    //     panic!("TODO")
    // }
    // pub trait AbstractExecution {
    //     fn mfp(&self);
    // }

    // pub fn execute(block: &lir::Block, domain_type: &domain::DomainType) {
    //     // execute a block
    //     panic!("TODO")
    // }

    pub fn mfp_constant(prog: &lir::Program) -> HashMap<lir::Block, store::ConstantStore> {
        // MFP (Meet For All Paths) worklist algorithm
        let mut bb2store: HashMap<lir::Block, store::ConstantStore> = HashMap::new();
        let cfg = cfg::ControlFlowGraph::from_program(prog);
        let mut worklist = cfg.to_sequence();
        let mut entry_store = store::ConstantStore::new();
        // set content of entry_store
        let globals = prog.get_all_globals();
        let parameters = prog.get_all_parameters();
        for global in globals {
            entry_store.set(global, domain::Constant::Top);
        }
        for parameter in parameters {
            // set all "int" parameters to Top; otherwise set to Bottom
            match parameter.typ {
                lir::Type::Int => entry_store.set(parameter, domain::Constant::Top),
                _ => entry_store.set(parameter, domain::Constant::Bottom),
            }
        }
        // bb2store.insert(cfg.get_first_block().unwrap().clone(), entry_store);


        bb2store
    }

    pub fn execute_constant(block: &lir::Block, store: &store::ConstantStore) {
        // execute a block
        panic!("TODO")
    }
}
