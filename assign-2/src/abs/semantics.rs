use crate::lir;

pub trait AbstractSemantics {
    fn is_bottom(&self) -> bool;
    fn is_top(&self) -> bool;
    fn join(&self, other: &Self) -> Self;
    fn arith(&self, other: &Self, op: &lir::ArithOp) -> Self;
    fn cmp(&self, other: &Self, op: &lir::RelaOp) -> Self;
}
