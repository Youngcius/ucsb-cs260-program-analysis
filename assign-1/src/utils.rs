/*
Utils functions
*/

use crate::{abs::semantics::AbstractSemantics, store};
use std::collections::HashMap;

pub fn display_bb2store<T>(bb2store: HashMap<String, store::Store<T>>)
where
    T: std::fmt::Display + Clone + AbstractSemantics,
{
    // blocks are printed in alphabetical order
    // variables are printed in alphabetical order for each block
    let mut bbs: Vec<String> = bb2store.keys().cloned().collect();
    bbs.sort();
    for bb in bbs {
        println!("{}:", bb);
        println!("{}", bb2store.get(&bb).unwrap());
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_display_constant_bb2store() {
        use super::*;
        use crate::abs::domain;
        use crate::lir;
        /*

           bb1:
           l -> Top
           n -> Top
           t1 -> Top
           t2 -> Top
           t3 -> Top

           bb2:
           l -> Top
           n -> ⊥
           t1 -> Top
           t2 -> Top
           t3 -> Top

           entry:
           l -> Top
           n -> 1

        */
        let mut bb2store: HashMap<String, store::ConstantStore> = HashMap::new();

        let mut store1 = store::ConstantStore::new();
        store1.set(lir::Variable::new("t1"), domain::Constant::Top);
        store1.set(lir::Variable::new("t2"), domain::Constant::Top);
        store1.set(lir::Variable::new("t3"), domain::Constant::Top);
        store1.set(lir::Variable::new("l"), domain::Constant::Top);
        store1.set(lir::Variable::new("n"), domain::Constant::Top);

        let mut store2 = store::ConstantStore::new();
        store2.set(lir::Variable::new("t1"), domain::Constant::Top);
        store2.set(lir::Variable::new("t2"), domain::Constant::Top);
        store2.set(lir::Variable::new("t3"), domain::Constant::Top);
        store2.set(lir::Variable::new("l"), domain::Constant::Top);
        store2.set(lir::Variable::new("n"), domain::Constant::Bottom);

        let mut entry_store = store::ConstantStore::new();
        entry_store.set(lir::Variable::new("l"), domain::Constant::Top);
        entry_store.set(lir::Variable::new("n"), domain::Constant::CInt(1));

        bb2store.insert("bb1".to_string(), store1);
        bb2store.insert("bb2".to_string(), store2);
        bb2store.insert("entry".to_string(), entry_store);
        display_bb2store(bb2store);
    }
}
