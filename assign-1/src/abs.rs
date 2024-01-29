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
    }

    impl std::fmt::Display for Interval {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Bottom => write!(f, "None"),
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

    impl Interval {
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
        fn join(&self, other: &Self) -> Self;
        fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self;
        fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self;
    }
}

pub mod execution {

    use std::collections::HashMap;

    use super::domain;
    use super::semantics::AbstractSemantics;
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

    // fn mfp<T>(prog: &lir::Program) -> HashMap<lir::Block, store::Store<T>> {
    //     let mut bb2store: HashMap<lir::Block, store::Store<T>> = HashMap::new();
    //     let cfg = cfg::ControlFlowGraph::from_program(prog);
    //     let mut worklist = cfg.to_sequence();
    //     let mut entry_store = store::Store::new();
    //     // set content of entry_store
    //     bb2store
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
            entry_store.set(global.clone(), domain::Constant::Top);
        }
        for parameter in parameters {
            // set all "int" parameters to Top; otherwise set to Bottom
            match parameter.typ {
                lir::Type::Int => entry_store.set(parameter.clone(), domain::Constant::Top),
                _ => entry_store.set(parameter.clone(), domain::Constant::Bottom),
            }
        }
        // bb2store.insert(cfg.get_first_block().unwrap().clone(), entry_store);

        bb2store
    }

    pub fn execute_constant(block: &lir::Block, store: &store::ConstantStore) {
        // execute a block
        panic!("TODO")
    }

    pub fn mfp_interval(prog: &lir::Program) -> HashMap<lir::Block, store::IntervalStore> {
        // MFP (Meet For All Paths) worklist algorithm
        let mut bb2store: HashMap<lir::Block, store::IntervalStore> = HashMap::new();
        let cfg = cfg::ControlFlowGraph::from_program(prog);
        let mut worklist = cfg.to_sequence();
        let mut entry_store = store::IntervalStore::new();
        // set content of entry_store
        let globals = prog.get_all_globals();
        let parameters = prog.get_all_parameters();
        for global in globals {
            entry_store.set(global.clone(), domain::Interval::Top);
        }
        for parameter in parameters {
            // set all "int" parameters to Top; otherwise set to Bottom
            match parameter.typ {
                lir::Type::Int => entry_store.set(parameter.clone(), domain::Interval::Top),
                _ => entry_store.set(parameter.clone(), domain::Interval::Bottom),
            }
        }
        // bb2store.insert(cfg.get_first_block().unwrap().clone(), entry_store);

        bb2store
    }
    pub fn execute_interval(block: &lir::Block, store: &mut store::IntervalStore) {
        // execute a block
        panic!("TODO")
    }

    // pub fn exe_instr<T>(instr: &lir::Instruction, store: &mut store::Store<T>) {
    //     // execute an instruction on the store
    //     // current support: no-function no-pointer
    //     // op: Operand::CInt(i32), or Operand::Var { name: String, typ: Type::Int, scope: ...}
    //     match instr {
    //         lir::Instruction::AddrOf { lhs, rhs } => {
    //             // {"AddrOf": {"lhs": "xxx", "rhs": "xxx"}}
    //             let rhs_val = store.get(rhs).unwrap();
    //             store.set(lhs.clone(), rhs_val.clone());
    //         }
    //         lir::Instruction::Alloc { lhs, num, id } => {
    //             // {"Alloc": {"lhs": "xxx", "num": "xxx", "id": "xxx"}}
    //             let num_val = store.get(num).unwrap();
    //             let id_val = store.get(id).unwrap();
    //             store.set(lhs.clone(), id_val.clone());
    //         }
    //         lir::Instruction::Copy { lhs, op } => {
    //             // {"Copy": {"lhs": "xxx", "op": "xxx"}}
    //             let op_val = store.get(op).unwrap();
    //             store.set(lhs.clone(), op_val.clone());
    //             match op {
    //                 lir::Operand::Var(var) => {

    //                 },
    //                 lir::Operand::CInt(c) => {
    //                     store.set<domain::Constant>(lhs.clone(), domain::Constant::CInt(*c));
    //                 }
    //             }
    //         }
    //         lir::Instruction::Gep { lhs, src, idx } => {
    //             // {"Gep": {"lhs": "xxx", "src": "xxx", "idx": "xxx"}}
    //             let src_val = store.get(src).unwrap();
    //             let idx_val = store.get(idx).unwrap();
    //             store.set(lhs.clone(), src_val.clone());
    //         }
    //         lir::Instruction::Arith { lhs, aop, op1, op2 } => {
    //             // {"Arith": {"lhs": "xxx", "aop": "xxx", "op1": "xxx", "op2": "xxx"}}
    //             let op1_val = store.get(op1).unwrap();
    //             let op2_val = store.get(op2).unwrap();
    //             let res_val = op1_val.arith(op2_val, aop);
    //             store.set(lhs.clone(), res_val);
    //         }
    //         lir::Instruction::Load { lhs, src } => {
    //             // {"Load": {"lhs": "xxx", "src": "xxx"}
    //             let src_val = store.get(src).unwrap();
    //             store.set(lhs.clone(), src_val.clone());
    //         }
    //         lir::Instruction::Store { dst, op } => {
    //             // {"Store": {"dst": "xxx", "op": "xxx"}}
    //             let op_val = store.get(op).unwrap();
    //             store.set(dst.clone(), op_val.clone());
    //         }
    //         lir::Instruction::Gfp { lhs, src, field } => {
    //             // {"Gfp": {"lhs": "xxx", "src": "xxx", "field": "xxx"}}
    //             let src_val = store.get(src).unwrap();
    //             let field_val = store.get(field).unwrap();
    //             store.set(lhs.clone(), src_val.clone());
    //         }
    //         lir::Instruction::Cmp { lhs, rop, op1, op2 } => {
    //             // {"Cmp": {"lhs": "xxx", "rop": "xxx", "op1": "xxx", "op2": "xxx"}}
    //             let op1_val = store.get(op1).unwrap();
    //             let op2_val = store.get(op2).unwrap();
    //             let res_val = op1_val.cmp(op2_val, rop);
    //             store.set(lhs.clone(), res_val);
    //         }
    //         lir::Instruction::CallExt {
    //             lhs,
    //             ext_callee,
    //             args,
    //         } => {
    //             // {"CallExt": {"lhs": "xxx", "ext_callee": "xxx", "args": ["xxx", "xxx"]}}
    //             let mut arg_vals: Vec<T> = Vec::new();
    //             for arg in args {
    //                 arg_vals.push(store.get(arg).unwrap().clone());
    //             }
    //         }
    //     }
    // }

    // pub fn exe_instr_const<T>(instr: &lir::Instruction, store: &mut store::ConstantStore) {
    //     // execute an instruction on the store
    //     // current support: no-function no-pointer
    //     // op: Operand::CInt(i32), or Operand::Var { name: String, typ: Type::Int, scope: ...}
    //     // TODO: store 是一个全局唯一的变量吗？？？还是每个block对应的store不同？？？
    //     match instr {
    //         lir::Instruction::AddrOf { lhs, rhs } => {
    //             // {"AddrOf": {"lhs": "xxx", "rhs": "xxx"}}
    //             let rhs_val = store.get(rhs).unwrap();
    //             store.set(lhs.clone(), rhs_val.clone());
    //         }
    //         lir::Instruction::Alloc { lhs, num, id } => {
    //             // {"Alloc": {"lhs": "xxx", "num": "xxx", "id": "xxx"}}
    //             let num_val = store.get(num).unwrap();
    //             let id_val = store.get(id).unwrap();
    //             store.set(lhs.clone(), id_val.clone());
    //         }
    //         lir::Instruction::Copy { lhs, op } => {
    //             // {"Copy": {"lhs": "xxx", "op": "xxx"}}
    //             match op {
    //                 lir::Operand::Var(var) => {
    //                     store.set(lhs.clone(), store.get(var).unwrap().clone());
    //                 }
    //                 lir::Operand::CInt(c) => {
    //                     store.set(lhs.clone(), domain::Constant::CInt(*c));
    //                 }
    //             }
    //         }
    //         lir::Instruction::Gep { lhs, src, idx } => {
    //             // {"Gep": {"lhs": "xxx", "src": "xxx", "idx": "xxx"}}
    //             let src_val = store.get(src).unwrap();
    //             let idx_val = store.get(idx).unwrap();
    //             store.set(lhs.clone(), src_val.clone());
    //         }
    //         lir::Instruction::Arith { lhs, aop, op1, op2 } => {
    //             // {"Arith": {"lhs": "xxx", "aop": "xxx", "op1": "xxx", "op2": "xxx"}}
    //             let res_val: domain::Constant;
    //             match (op1, op2) {
    //                 (lir::Operand::Var(var1), lir::Operand::Var(var2)) => {
    //                     let op1_val = store.get(var1).unwrap();
    //                     let op2_val = store.get(var2).unwrap();
    //                     res_val = op1_val.arith(op2_val, aop);
    //                 }
    //                 (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
    //                     let op1_val = store.get(var).unwrap();
    //                     let op2_val = domain::Constant::CInt(*c);
    //                     res_val = op1_val.arith(&op2_val, aop);
    //                 }
    //                 (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
    //                     let op1_val = domain::Constant::CInt(*c);
    //                     let op2_val = store.get(var).unwrap();
    //                     res_val = op1_val.arith(op2_val, aop);
    //                 }
    //                 (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
    //                     let op1_val = domain::Constant::CInt(*c1);
    //                     let op2_val = domain::Constant::CInt(*c2);
    //                     res_val = op1_val.arith(&op2_val, aop);
    //                 }
    //             }
    //             store.set(lhs.clone(), res_val);
    //         }
    //         lir::Instruction::Load { lhs, src } => {
    //             // {"Load": {"lhs": "xxx", "src": "xxx"}
    //             let src_val = store.get(src).unwrap();
    //             store.set(lhs.clone(), src_val.clone());
    //         }
    //         lir::Instruction::Store { dst, op } => {
    //             // {"Store": {"dst": "xxx", "op": "xxx"}}
    //             let op_val = store.get(op).unwrap();
    //             store.set(dst.clone(), op_val.clone());
    //         }
    //         lir::Instruction::Gfp { lhs, src, field } => {
    //             // {"Gfp": {"lhs": "xxx", "src": "xxx", "field": "xxx"}}
    //             let src_val = store.get(src).unwrap();
    //             let field_val = store.get(field).unwrap();
    //             store.set(lhs.clone(), src_val.clone());
    //         }
    //         lir::Instruction::Cmp { lhs, rop, op1, op2 } => {
    //             // {"Cmp": {"lhs": "xxx", "rop": "xxx", "op1": "xxx", "op2": "xxx"}}
    //             let res_val: domain::Constant;
    //             match (op1, op2) {
    //                 (lir::Operand::Var(var1), lir::Operand::Var(var2)) => {
    //                     let op1_val = store.get(var1).unwrap();
    //                     let op2_val = store.get(var2).unwrap();
    //                     res_val = op1_val.cmp(op2_val, rop);
    //                 }
    //                 (lir::Operand::Var(var), lir::Operand::CInt(c)) => {
    //                     let op1_val = store.get(var).unwrap();
    //                     let op2_val = domain::Constant::CInt(*c);
    //                     res_val = op1_val.cmp(&op2_val, rop);
    //                 }
    //                 (lir::Operand::CInt(c), lir::Operand::Var(var)) => {
    //                     let op1_val = domain::Constant::CInt(*c);
    //                     let op2_val = store.get(var).unwrap();
    //                     res_val = op1_val.cmp(op2_val, rop);
    //                 }
    //                 (lir::Operand::CInt(c1), lir::Operand::CInt(c2)) => {
    //                     let op1_val = domain::Constant::CInt(*c1);
    //                     let op2_val = domain::Constant::CInt(*c2);
    //                     res_val = op1_val.cmp(&op2_val, rop);
    //                 }
    //             }
    //             store.set(lhs.clone(), res_val);
    //         }
    //         lir::Instruction::CallExt {
    //             lhs,
    //             ext_callee,
    //             args,
    //         } => {
    //             // {"CallExt": {"lhs": "xxx", "ext_callee": "xxx", "args": ["xxx", "xxx"]}}
    //             let mut arg_vals: Vec<T> = Vec::new();
    //             for arg in args {
    //                 arg_vals.push(store.get(arg).unwrap().clone());
    //             }
    //         }
    //     }
    // }
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

        assert_eq!(bottom.to_string(), "None");
        assert_eq!(top.to_string(), "(NegInf, PosInf)");
        assert_eq!(range.to_string(), "[1, 2]");
    }
}
