/*
Abstract domain, abstract semantics, and abstract execution.
*/



    use crate::lir;


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
