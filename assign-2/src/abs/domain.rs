use super::semantics::AbstractSemantics;
use crate::lir;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Bottom,
    Top,
    CInt(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramPoint {
    Bottom,
    Top,
    ProgramPointSet(HashSet<lir::ProgramPoint>),
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

// impl ProgramPoint {
//     pub fn from_pp(pp: lir::ProgramPoint) -> Self {
//         let mut pps = HashSet::new();
//         pps.insert(pp);
//         ProgramPoint::ProgramPointSet(pps)
//     }
//     pub fn from_pps(pps: Vec<lir::ProgramPoint>) -> Self {
//         let mut pps_set = HashSet::new();
//         for pp in pps {
//             pps_set.insert(pp);
//         }
//         ProgramPoint::ProgramPointSet(pps_set)
//     }
// }

impl std::fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bottom => write!(f, "⊥"),
            Self::Top => write!(f, "Top"),
            Self::ProgramPointSet(pps) => {
                let mut pps = pps.iter().map(|pp| pp.to_string()).collect::<Vec<String>>();
                pps.sort();
                write!(f, "{:?}", pps)
            }
        }
    }
}

impl AbstractSemantics for ProgramPoint {
    fn is_bottom(&self) -> bool {
        if let ProgramPoint::Bottom = self {
            true
        } else {
            false
        }
    }
    fn is_top(&self) -> bool {
        if let ProgramPoint::Top = self {
            true
        } else {
            false
        }
    }
    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (ProgramPoint::Bottom, _) => other.clone(),
            (_, ProgramPoint::Bottom) => self.clone(),
            (ProgramPoint::Top, _) => ProgramPoint::Top,
            (_, ProgramPoint::Top) => ProgramPoint::Top,
            (ProgramPoint::ProgramPointSet(pps1), ProgramPoint::ProgramPointSet(pps2)) => {
                let mut pps = pps1.clone();
                pps.extend(pps2.clone());
                ProgramPoint::ProgramPointSet(pps)
            }
        }
    }
    fn arith(&self, _other: &Self, _op: &lir::ArithOp) -> Self {
        panic!("ProgramPoint does not support arithmetic operations")
    }
    fn cmp(&self, _other: &Self, _op: &lir::RelaOp) -> Self {
        panic!("ProgramPoint does not support comparison operations")
    }
}
