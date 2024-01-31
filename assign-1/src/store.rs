/*
Store is about the result of abstract execution on a program.
*/
use crate::abs::domain;
use crate::abs::semantics::AbstractSemantics;
use crate::lir;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Store<T> {
    status: HashMap<lir::Variable, T>,
}

impl<T> Store<T>
where
    T: Clone + AbstractSemantics,
{
    pub fn new() -> Self {
        Self {
            status: HashMap::new(),
        }
    }

    pub fn join(&self, other: &Self) -> Self {
        let mut res = Self::new();
        // insert all <k,v> from self to res
        for (var, domain) in self.status.iter() {
            res.status.insert(var.clone(), domain.clone());
        }
        // join/insert all <k,v> from other to res
        for (var, domain) in other.status.iter() {
            if res.status.contains_key(var) {
                // join
                res.status
                    .insert(var.clone(), res.status.get(var).unwrap().join(domain));
            } else {
                // insert
                res.status.insert(var.clone(), domain.clone());
            }
        }
        res
    }

    pub fn get(&self, var: &lir::Variable) -> Option<&T> {
        self.status.get(var)
    }

    pub fn get_by_name(&self, var_name: &str) -> Option<&T> {
        for (var, domain) in &self.status {
            if var.name == var_name {
                return Some(domain);
            }
        }
        None
    }

    pub fn get_variables(&self) -> Vec<lir::Variable> {
        self.status.keys().cloned().collect()
    }

    pub fn get_var_names(&self) -> Vec<String> {
        self.status.keys().map(|v| v.name.clone()).collect()
    }

    pub fn set(&mut self, var: lir::Variable, domain: T) {
        self.status.insert(var.clone(), domain);
    }
}

impl<T> std::fmt::Display for Store<T>
where
    T: std::fmt::Display + AbstractSemantics + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print variable names in alphabetical order
        let mut var_names = self.get_var_names();
        var_names.sort();
        for var_name in &var_names {
            write!(
                f,
                "{} -> {}\n",
                var_name,
                self.get_by_name(var_name).unwrap()
            )?;
        }
        Ok(())
    }
}

pub type ConstantStore = Store<domain::Constant>;
pub type IntervalStore = Store<domain::Interval>;

#[cfg(test)]
mod test {
    #[test]
    fn test_generic_construction() {
        use super::Store;
        use crate::abs::domain::Constant;
        use crate::lir;
        let mut store = Store::<Constant>::new();
        let var = lir::Variable::new("x");
        let value = Constant::CInt(100);
        store.set(var.clone(), value.clone());
        println!("{:?}", store.get(&var).unwrap());
        // assert_eq!(store.get(&var).unwrap(), 1);
    }

    #[test]
    fn test_get_set() {
        use super::Store;
        use crate::abs::domain::Constant;
        use crate::lir;
        let mut store = Store::<Constant>::new();
        let var = lir::Variable::new("x");
        let var1 = lir::Variable::new("y");
        let value = Constant::CInt(100);
        store.set(var.clone(), value.clone());
        println!("getting var: {:?}", store.get(&var).unwrap());
        // println!("getting var1: {:?}", store.get(&var1).unwrap());

        if let Some(val) = store.get(&var) {
            // 处理存在的值
            println!("Value: {:?}", val);
        } else {
            // 处理不存在的键
            println!("Key not found");
        }

        if let Some(val) = store.get(&var1) {
            // 处理存在的值
            println!("Value: {:?}", val);
        } else {
            // 处理不存在的键
            println!("Key not found");
        }

        // assert_eq!(store.get(&var).unwrap(), 1);
    }
}
