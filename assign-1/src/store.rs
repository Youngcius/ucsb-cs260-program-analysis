/*
Store is about the result of abstract execution on a program.
*/
use crate::abs::domain;
use crate::lir;
use std::collections::HashMap;

pub struct ConstantStore {
    status: HashMap<lir::Variable, domain::Constant>,
}

impl ConstantStore{
    pub fn new() -> Self {
        Self {
            status: HashMap::new(),
        }
    }

    pub fn join(&self, other: &Self) -> Self {
        let res = Self::new();
        let aa: HashMap<&i32, i32> = HashMap::new();

        res
    }

    pub fn get(&self, var: &lir::Variable) -> Option<&domain::Constant> {
        self.status.get(var)
    }

    pub fn set(&mut self, var: &lir::Variable, constant: domain::Constant) {
        self.status.insert(var.clone(), constant);
    }

    

    // pub fn get(&self, var: &lir::Variable) -> domain::Constant {
    //     match self.status.get(var) {
    //         Some(constant) => constant.clone(),
    //         None => domain::Constant::Bottom,
    //     }
    // }


}
