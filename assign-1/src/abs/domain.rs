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
    Bottom,
    Top,
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
    Bottom,
    Top,
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
