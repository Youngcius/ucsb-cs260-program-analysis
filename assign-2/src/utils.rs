/*
Utils functions
*/

use crate::lir;
use crate::{abs::semantics::AbstractSemantics, store};
use std::collections::HashMap;
use crate::abs::domain;

#[macro_export]
macro_rules! hashset {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };

}

pub fn display_bb2store<T>(bb2store: &HashMap<String, store::Store<T>>)
where
    T: std::fmt::Display + Clone + AbstractSemantics,
{
    // blocks are printed in alphabetical order
    // variables are printed in alphabetical order for each block
    let mut bbs: Vec<String> = bb2store.keys().cloned().collect();
    bbs.sort();
    for bb in bbs {
        // if bb == "dummy_entry" || bb == "dummy_exit" {
        //     continue;
        // }
        if bb2store.get(&bb).unwrap().len() > 0 || bb == "entry" {
            println!("{}:", bb);
            println!("{}", bb2store.get(&bb).unwrap());
        }
    }
}

pub fn display_rdef_solution(solution: &HashMap<String, domain::ProgramPoint>)
{
    let mut keys: Vec<String>= solution.keys().cloned().collect();
    keys.sort();
    for k in keys {
        if let domain::ProgramPoint::Bottom = solution.get(&k).unwrap() {
            continue;
        }
        if let domain::ProgramPoint::ProgramPointSet(pps) = solution.get(&k).unwrap() {
            if pps.is_empty() {
                continue;
            }
        }
        // if let domain::ProgramPoint::ProgramPointSet(HashMap::new()) = solution.get(&k).unwrap() {
        //     continue;
        // }
        println!("{} -> {}", k, solution.get(&k).unwrap());
    }
}

pub fn able_to_reach_int(to: &Box<lir::Type>) -> bool {
    // "to" is the target pointed by a pointer
    /*
        pub enum Type {
        Int,                         // "Int"
        Struct(String),              // {"Struct": "xxx"}
        Function(Box<FunctionType>), // {"Function": "xxx"}
        Pointer(Box<Type>),          // {"Pointer": "xxx"}
    }
     */
    // we want to know if we can reach an integer from "to" (e.g., &&&int )
    match to.as_ref() {
        lir::Type::Int => true,
        lir::Type::Pointer(p) => able_to_reach_int(p),
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::abs::domain;
    use crate::lir;
    use crate::store::ConstantStore;

    #[test]
    fn test_display_constant_bb2store() {
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
        let mut bb2store: HashMap<String, ConstantStore> = HashMap::new();

        let mut store1 = ConstantStore::new();
        store1.set(lir::Variable::new("t1"), domain::Constant::Top);
        store1.set(lir::Variable::new("t2"), domain::Constant::Top);
        store1.set(lir::Variable::new("t3"), domain::Constant::Top);
        store1.set(lir::Variable::new("l"), domain::Constant::Top);
        store1.set(lir::Variable::new("n"), domain::Constant::Top);

        let mut store2 = ConstantStore::new();
        store2.set(lir::Variable::new("t1"), domain::Constant::Top);
        store2.set(lir::Variable::new("t2"), domain::Constant::Top);
        store2.set(lir::Variable::new("t3"), domain::Constant::Top);
        store2.set(lir::Variable::new("l"), domain::Constant::Top);
        store2.set(lir::Variable::new("n"), domain::Constant::Bottom);

        let mut entry_store = ConstantStore::new();
        entry_store.set(lir::Variable::new("l"), domain::Constant::Top);
        entry_store.set(lir::Variable::new("n"), domain::Constant::CInt(1));

        bb2store.insert("bb1".to_string(), store1);
        bb2store.insert("bb2".to_string(), store2);
        bb2store.insert("entry".to_string(), entry_store);
        display_bb2store(&bb2store);
    }

    #[test]
    fn test_able_to_reach_int() {
        let i = lir::Type::Int;
        let p = lir::Type::Pointer(Box::new(lir::Type::Int));
        let pp = lir::Type::Pointer(Box::new(lir::Type::Pointer(Box::new(lir::Type::Int))));
        let ppp = lir::Type::Pointer(Box::new(lir::Type::Pointer(Box::new(lir::Type::Pointer(
            Box::new(lir::Type::Int),
        )))));
        println!("i can reach int?: {}", able_to_reach_int(&Box::new(i)));
        println!("p can reach int?: {}", able_to_reach_int(&Box::new(p)));
        println!("pp can reach int?: {}", able_to_reach_int(&Box::new(pp)));
        println!("ppp can reach int?: {}", able_to_reach_int(&Box::new(ppp)));
    }
}
