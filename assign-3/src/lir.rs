/*
Low-level intermediate representation (LIR) for program analysis.
*/
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub structs: HashMap<String, Vec<Field>>,
    pub globals: Vec<Variable>,
    pub functions: HashMap<String, Function>, // function definitions
    pub externs: HashMap<String, Type>,       // external function declarations
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub typ: Type,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,                         // "Int"
    Struct(String),              // {"Struct": "xxx"}
    Function(Box<FunctionType>), // {"Function": "xxx"}
    Pointer(Box<Type>),          // {"Pointer": "xxx"}
}

impl Type {
    pub fn reachable_types(&self, prog: &Program) -> HashSet<Type> {
        let mut reachable = HashSet::new();
        self.reaching_types(prog, &mut reachable);
        reachable
    }

    pub fn reaching_types(&self, prog: &Program, res: &mut HashSet<Type>) {
        // return the set of types reachable via pointer dereference and/or struct field access from the given type,
        // excluding struct and function types included in its field or via pointer dereference
        match self {
            Type::Int => {
                res.insert(Type::Int);
            }
            Type::Struct(struct_name) => {
                // reachable.insert(Type::Struct("".to_string()));
                // println!("Struct type: {}", struct_name);
                prog.structs.get(struct_name).unwrap().iter().for_each(|f| {
                    // println!("Struct field: {:#?}", f);
                    if !res.contains(&f.typ) {
                        f.typ.reaching_types(prog, res);
                    }
                });
            }
            Type::Function(_) => {
                // reachable.insert(Type::Function(Box::new(FunctionType {
                //     ret_ty: None,
                //     param_ty: Vec::new(),
                // })));
            }
            Type::Pointer(t) => {
                // if let Type::Function(_) = **t {
                //     return;
                // }
                if !res.contains(&**t) {
                    res.insert(Type::Pointer(t.clone()));
                    t.reaching_types(prog, res);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    // it could be as parameter, local variable, or global variable
    pub name: String,
    pub typ: Type,
    pub scope: Option<String>,
}

impl Variable {
    pub fn new(name: &str) -> Variable {
        // Only support Int type for now
        Variable {
            name: name.to_string(),
            typ: Type::Int,
            scope: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub id: String,
    pub ret_ty: Option<Type>,
    pub params: Vec<Variable>,
    pub locals: Vec<Variable>,
    pub body: HashMap<String, Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub ret_ty: Option<Type>,
    pub param_ty: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub id: String,
    pub insts: Vec<Instruction>,
    pub term: Terminal,
}
impl Block {
    pub fn new(id: &str, term: &Terminal) -> Block {
        Block {
            id: id.to_string(),
            insts: Vec::new(),
            term: term.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    // 10 kinds of instructions
    AddrOf {
        // {"AddrOf": {"lhs": "xxx", "rhs": "xxx"}}
        lhs: Variable,
        rhs: Variable,
    },
    Alloc {
        // {"Alloc": {"lhs": "xxx", "num": "xxx", "id": "xxx"}}
        lhs: Variable,
        num: Operand,
        id: Variable,
    },
    Copy {
        // {"Copy": {"lhs": "xxx", "op": "xxx"}}
        lhs: Variable,
        op: Operand,
    },
    Gep {
        // get-element-pointer, {"Gep": {"lhs": "xxx", "src": "xxx", "idx": "xxx"}}
        lhs: Variable,
        src: Variable,
        idx: Operand,
    },
    Arith {
        // {"Arith": {"lhs": "xxx", "aop": "xxx", "op1": "xxx", "op2": "xxx"}}
        lhs: Variable,
        aop: ArithOp,
        op1: Operand,
        op2: Operand,
    }, //
    Load {
        // {"Load": {"lhs": "xxx", "src": "xxx"}
        lhs: Variable,
        src: Variable,
    },
    Store {
        // {"Store": {"dst": "xxx", "op": "xxx"}}
        dst: Variable,
        op: Operand,
    },
    Gfp {
        // {"Gfp": {"lhs": "xxx", "src": "xxx", "field": "xxx"}}
        lhs: Variable,
        src: Variable,
        field: Variable,
    },
    Cmp {
        // {"Cmp": {"lhs": "xxx", "rop": "xxx", "op1": "xxx", "op2": "xxx"}}
        lhs: Variable,
        rop: RelaOp,
        op1: Operand,
        op2: Operand,
    },
    CallExt {
        // {"CallExt": {"lhs": "xxx", "ext_callee": "xxx", "args": ["xxx", "xxx"]}}
        lhs: Option<Variable>,
        ext_callee: String,
        args: Vec<Operand>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArithOp {
    // arithmetic operators
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelaOp {
    // relational operators
    Neq,
    Eq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminal {
    // a terminal signals the end of a basic block and is one of
    Jump(String), // {"Jump": "bb1"}
    Branch {
        // {"Branch": {"cond": "xxx", "tt": "xxx", "ff": "xxx"}}
        cond: Operand,
        tt: String,
        ff: String,
    },
    Ret(Option<Operand>), // {"Ret": "xxx"}
    CallDirect {
        // {"CallDirect": {"lhs": "xxx", "callee": "xxx", "args": ["xxx", "xxx"], "next_bb": "xxx"}}
        lhs: Option<Variable>,
        callee: String,
        args: Vec<Operand>,
        next_bb: String,
    },
    CallIndirect {
        // {"CallIndirect": {"lhs": "xxx", "callee": "xxx", "args": ["xxx", "xxx"], "next_bb": "xxx"}}
        lhs: Option<Variable>,
        callee: Variable,
        args: Vec<Operand>,
        next_bb: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    // an operand is either a variable or a constant
    Var(Variable),
    CInt(i32),
}

// Additional LIR components for analysis other than parsing

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramPoint {
    pub block: String,      // block id
    pub location: Location, // i-th instruction or terminal
    // pub using: HashSet<Variable>, // variables used are extracted from Operand objects
    // pub defining: Option<Variable>,
    pub instr: Option<Instruction>,
    pub term: Option<Terminal>,
}

impl std::fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.location {
            Location::Instruction(i) => write!(f, "{}.{}", self.block, i),
            Location::Terminal => write!(f, "{}.term", self.block),
        }
    }
}

impl PartialOrd for ProgramPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let block_cmp = self.block.cmp(&other.block);
        match block_cmp {
            std::cmp::Ordering::Equal => match (&self.location, &other.location) {
                (Location::Instruction(i1), Location::Instruction(i2)) => i1.partial_cmp(i2),
                (Location::Instruction(_), Location::Terminal) => Some(std::cmp::Ordering::Less),
                (Location::Terminal, Location::Instruction(_)) => Some(std::cmp::Ordering::Greater),
                (Location::Terminal, Location::Terminal) => Some(std::cmp::Ordering::Equal),
            },
            _ => Some(block_cmp),
        }
    }
}

impl Ord for ProgramPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// impl ProgramPoint {
//     pub fn from_instr(bb_label: &str, instr: &Instruction, idx: usize) -> Self {
//         let mut using = HashSet::new();
//         let mut defining = None;
//         match instr {
//             Instruction::AddrOf { lhs, rhs: _ } => {
//                 defining = Some(lhs.clone());
//             }
//             Instruction::Arith { lhs, aop, op1, op2 } => {
//                 if let Operand::Var(v) = op1 {
//                     using.insert(v);
//                 }
//                 if let Operand::Var(v) = op2 {
//                     using.insert(v);
//                 }
//                 defining = Some(lhs.clone());
//             }
//         }
//         Self {
//             block: bb_label.to_string(),
//             location: Location::Instruction(idx),
//             using,
//             defining,
//         }
//     }

//     pub fn from_term(bb_label: &str, term: &Terminal) -> Self {
//         Self {
//             block: bb_label.to_string(),
//             location: Location::Terminal,
//             using,
//             defining,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Location {
    Instruction(usize),
    Terminal,
}

// #[derive(Debug, Clone)]
// pub struct LocalVariable {
//     pub func: String,
//     pub var: Variable,
// }
//
// pub type GlobalVariable = Variable;

// TODO: define fake variable
// #[derive(Debug, Clone)]
// pub struct FakeVariable {
//     pub name: String,
//     pub typ: Type,
// }

// pub type FakeVariable = Variable;

impl Program {
    pub fn new() -> Program {
        Program {
            structs: HashMap::new(),
            globals: Vec::new(),
            functions: HashMap::new(),
            externs: HashMap::new(),
        }
    }

    pub fn get_int_globals(&self) -> Vec<Variable> {
        let global_ints: Vec<Variable> = self
            .globals
            .iter()
            .filter(|v| match v.typ {
                Type::Int => true,
                _ => false,
            })
            .cloned()
            .collect();
        global_ints
    }

    pub fn get_ptr_globals(&self) -> Vec<Variable> {
        let global_ptrs: Vec<Variable> = self
            .globals
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        global_ptrs
    }

    pub fn get_int_parameters(&self, func_name: &str) -> Vec<Variable> {
        let func = self.functions.get(func_name).unwrap();
        let param_ints = func
            .params
            .iter()
            .filter(|v| match v.typ {
                Type::Int => true,
                _ => false,
            })
            .cloned()
            .collect();
        param_ints
    }

    pub fn get_ptr_parameters(&self, func_name: &str) -> Vec<Variable> {
        let func = self.functions.get(func_name).unwrap();
        let param_ptrs = func
            .params
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        param_ptrs
    }

    pub fn get_int_locals(&self, func_name: &str) -> Vec<Variable> {
        let func = self.functions.get(func_name).unwrap();
        let local_ints = func
            .locals
            .iter()
            .filter(|v| match v.typ {
                Type::Int => true,
                _ => false,
            })
            .cloned()
            .collect();
        local_ints
    }

    pub fn get_ptr_locals(&self, func_name: &str) -> Vec<Variable> {
        let func = self.functions.get(func_name).unwrap();
        let local_ptrs = func
            .locals
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(_) => true,
                _ => false,
            })
            .cloned()
            .collect();
        local_ptrs
    }

    pub fn get_addrof_ints(&self, func_name: &str) -> Vec<Variable> {
        // TODO: whether it should include global_ints herein?
        let func = self.functions.get(func_name).unwrap();
        let mut addrof_ints = Vec::new();
        for block in func.body.values() {
            for inst in &block.insts {
                match inst {
                    Instruction::AddrOf { lhs: _, rhs } => {
                        if let Type::Int = rhs.typ {
                            if !addrof_ints.contains(rhs) {
                                addrof_ints.push(rhs.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        for var in self.get_int_globals() {
            if !addrof_ints.contains(&var) {
                addrof_ints.push(var);
            }
        }
        #[cfg(debug_assertions)]
        {
            println!("---------------------------------");
            println!("addrof_ints: {:?}", addrof_ints);
            println!("---------------------------------");
        }
        addrof_ints
    }

    pub fn get_addr_taken(&self, func_name: &str) -> Vec<Variable> {
        let mut ptrs = self.get_ptr_globals();
        ptrs.extend(self.get_ptr_locals(func_name));
        ptrs.extend(self.get_ptr_parameters(func_name));

        let mut reached_types = HashSet::new();
        for typ in ptrs.iter().map(|v| &v.typ) {
            // reached_types = reached_types.union(&typ.reachable_types(self)).cloned().collect();
            reached_types.extend(typ.reachable_types(self));
        }

        let fake_vars_set: HashSet<Variable> = reached_types
            .iter()
            .map(|t| Variable {
                name: "fake_var".to_string(),
                typ: t.clone(),
                scope: Some("fake".to_string()), // TODO: how to set its scope?
            })
            .collect();
        let fake_vars: Vec<Variable> = fake_vars_set.into_iter().collect();
        
        let mut addr_taken = Vec::new();
        addr_taken.extend(fake_vars);
        addr_taken.extend(self.get_addrof_ints(func_name));
        addr_taken
    }

    pub fn get_all_basic_blocks(&self) -> Vec<&Block> {
        let mut blocks = Vec::new();
        for func in self.functions.values() {
            for block in func.body.values() {
                blocks.push(block);
            }
        }
        blocks
    }

    pub fn parse_json(fname: &str) -> Program {
        let file = File::open(fname).expect("Failed to open file");
        let reader = BufReader::new(file);
        json::from_reader(reader).unwrap()
    }

    pub fn write_json(&self, fname: &str) {
        let file = File::create(fname).expect("Failed to create file");
        let writer = BufWriter::new(file);
        json::to_writer(writer, self).unwrap();
    }

    pub fn get_num_fields(&self) -> u32 {
        self.structs.values().map(|s| s.len()).sum::<usize>() as u32
    }
    pub fn get_num_functions(&self) -> u32 {
        self.functions
            .values()
            .filter(|f| f.ret_ty.is_some())
            .count() as u32
    }
    pub fn get_num_func_params(&self) -> u32 {
        self.functions
            .values()
            .map(|f: &Function| f.params.len())
            .sum::<usize>() as u32
    }
    pub fn get_num_locals(&self) -> u32 {
        self.functions
            .values()
            .map(|f: &Function| f.locals.len())
            .sum::<usize>() as u32
    }
    pub fn get_num_basic_blocks(&self) -> u32 {
        self.functions
            .values()
            .map(|f: &Function| f.body.len())
            .sum::<usize>() as u32
    }
    pub fn get_num_instructions(&self) -> u32 {
        self.functions
            .values()
            .map(|f: &Function| f.body.values().map(|b| b.insts.len()).sum::<usize>())
            .sum::<usize>() as u32
    }
    pub fn get_num_terminals(&self) -> u32 {
        self.functions
            .values()
            .map(|f: &Function| {
                f.body
                    .values()
                    .map(|b| match &b.term {
                        Terminal::Jump(_) => 1,
                        Terminal::Branch { .. } => 1,
                        Terminal::Ret(_) => 1,
                        Terminal::CallDirect { .. } => 1,
                        Terminal::CallIndirect { .. } => 1,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32
    }
    pub fn get_num_ints(&self) -> u32 {
        let mut num_ints = self
            .functions
            .values()
            .map(|f: &Function| {
                f.locals
                    .iter()
                    .filter(|v| match v.typ {
                        Type::Int => true,
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>() as u32;
        num_ints += self
            .globals
            .iter()
            .filter(|v| match v.typ {
                Type::Int => true,
                _ => false,
            })
            .count() as u32;
        num_ints
    }
    pub fn get_num_structs(&self) -> u32 {
        let mut num_structs = self
            .functions
            .values()
            .map(|f: &Function| {
                f.locals
                    .iter()
                    .filter(|v| match v.typ {
                        Type::Struct(_) => true,
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>() as u32;
        num_structs += self
            .globals
            .iter()
            .filter(|v| match v.typ {
                Type::Struct(_) => true,
                _ => false,
            })
            .count() as u32;

        num_structs
    }
    pub fn get_num_ptr_ints(&self) -> u32 {
        let mut num_ptr_ints = self
            .functions
            .values()
            .map(|f: &Function| {
                f.locals
                    .iter()
                    .filter(|v| match &v.typ {
                        Type::Pointer(t) => match **t {
                            Type::Int => true,
                            _ => false,
                        },
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>() as u32;
        num_ptr_ints += self
            .globals
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(t) => match **t {
                    Type::Int => true,
                    _ => false,
                },
                _ => false,
            })
            .count() as u32;
        num_ptr_ints
    }

    pub fn get_num_ptr_structs(&self) -> u32 {
        let mut num_ptr_structs = self
            .functions
            .values()
            .map(|f: &Function| {
                f.locals
                    .iter()
                    .filter(|v| match &v.typ {
                        Type::Pointer(t) => match **t {
                            Type::Struct(_) => true,
                            _ => false,
                        },
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>() as u32;
        num_ptr_structs += self
            .globals
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(t) => match **t {
                    Type::Struct(_) => true,
                    _ => false,
                },
                _ => false,
            })
            .count() as u32;
        num_ptr_structs
    }

    pub fn get_num_ptr_funcs(&self) -> u32 {
        let mut num_ptr_funcs = self
            .functions
            .values()
            .map(|f: &Function| {
                f.locals
                    .iter()
                    .filter(|v| match &v.typ {
                        Type::Pointer(t) => match **t {
                            Type::Function(_) => true,
                            _ => false,
                        },
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>() as u32;
        num_ptr_funcs += self
            .globals
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(t) => match **t {
                    Type::Function(_) => true,
                    _ => false,
                },
                _ => false,
            })
            .count() as u32;
        num_ptr_funcs
    }
    pub fn get_num_ptr_ptrs(&self) -> u32 {
        let mut num_ptr_ptrs = self
            .functions
            .values()
            .map(|f: &Function| {
                f.locals
                    .iter()
                    .filter(|v| match &v.typ {
                        Type::Pointer(t) => match **t {
                            Type::Pointer(_) => true,
                            _ => false,
                        },
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>() as u32;
        num_ptr_ptrs += self
            .globals
            .iter()
            .filter(|v| match &v.typ {
                Type::Pointer(t) => match **t {
                    Type::Pointer(_) => true,
                    _ => false,
                },
                _ => false,
            })
            .count() as u32;
        num_ptr_ptrs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_globals() {
        let prog_name = "./examples/json/lambda.json";
        let prog = Program::parse_json(prog_name);
        let global_ints = prog.get_int_globals();
        let param_ints = prog.get_int_parameters("main");
        let local_ints = prog.get_int_locals("main");
        // let gg = prog.globals;

        println!("======= int-type globals of {} =======", prog_name);
        for var in global_ints {
            println!("{:?}", var);
        }

        println!("======= int-type parameters of {} =======", prog_name);
        for var in param_ints {
            println!("{:?}", var);
        }

        println!("======= int-type locals of {} =======", prog_name);
        for var in local_ints {
            println!("{:?}", var);
        }
    }

    #[test]
    fn test_reachable_types() {
        let prog_name = "./examples/json/lambda.json";
        let prog = Program::parse_json(prog_name);

        let globals = &prog.globals;
        let structs = &prog.structs;

        for var in globals {
            let reachable = var.typ.reachable_types(&prog);
            println!("-------");
            println!("reachable types of {:#?} (globals):", var);
            for (i, t) in reachable.iter().enumerate() {
                println!("{}: {:#?}", i, t);
            }
            println!();
        }

        let func_name = "add";
        let func = prog.functions.get(func_name).unwrap();

        for var in &func.params {
            let reachable = var.typ.reachable_types(&prog);
            println!("-------");
            println!("reachable types of {:#?} (parameters):", var);
            for (i, t) in reachable.iter().enumerate() {
                println!("{}: {:#?}", i, t);
            }
            println!();
        }
    }

    #[test]
    fn test_get_variables() {
        let prog_name = "./examples/json/lambda.json";
        let prog = Program::parse_json(prog_name);
        let global_ints = prog.get_int_globals();
        let global_ptrs = prog.get_ptr_globals();
        let local_ints = prog.get_int_locals("add");
        let local_ptrs = prog.get_ptr_locals("add");
        let param_ints = prog.get_int_parameters("add");
        let param_ptrs = prog.get_ptr_parameters("add");

        println!("======= int-type globals of {} =======", prog_name);
        for var in global_ints {
            println!("{:?}", var);
        }
        println!();

        println!("======= pointer-type globals of {} =======", prog_name);
        for var in global_ptrs {
            println!("{:?}", var);
        }
        println!();

        println!("======= int-type parameters of {} =======", prog_name);
        for var in param_ints {
            println!("{:?}", var);
        }
        println!();

        println!("======= pointer-type parameters of {} =======", prog_name);
        for var in param_ptrs {
            println!("{:?}", var);
        }
        println!();

        println!("======= int-type locals of {} =======", prog_name);
        for var in local_ints {
            println!("{:?}", var);
        }
        println!();

        println!("======= pointer-type locals of {} =======", prog_name);
        for var in local_ptrs {
            println!("{:?}", var);
        }
    }

    #[test]
    fn test_get_addr_taken() {
        let prog_name = "./examples/json/lambda.json";
        let prog = Program::parse_json(prog_name);
        let addr_taken = prog.get_addr_taken("add");
        for (i, var) in addr_taken.iter().enumerate() {
            println!("{}: {:#?}", i, var);
        }

        let prog_name = "./demos/json/test8.json";
        let prog = Program::parse_json(prog_name);
        let addr_taken = prog.get_addr_taken("test");
        for (i, var) in addr_taken.iter().enumerate() {
            println!("{}: {:#?}", i, var);
        }
    }

    #[test]
    fn test_location_order() {
        let loc1 = Location::Instruction(1);
        let loc2 = Location::Instruction(2);
        let loc11 = Location::Instruction(11);
        let term = Location::Terminal;
        let mut v = vec![loc1, term, loc11, loc2];
        v.sort();
        println!("{:?}", v);
    }

    #[test]
    fn test_pps_order() {
        // let pp1 = ProgramPoint {
        //     block: "bb1".to_string(),
        //     location: Location::Instruction(1),
        //     instr: None,
        //     term: None,
        // };
        // let pp2 = ProgramPoint {
        //     block: "bb1".to_string(),
        //     location: Location::Instruction(2),
        //     instr: None,
        //     term: None,
        // };
        // let pp11 = ProgramPoint {
        //     block: "bb1".to_string(),
        //     location: Location::Instruction(11),
        //     instr: None,
        //     term: None,
        // };
        // let pp_term = ProgramPoint {
        //     block: "bb1".to_string(),
        //     location: Location::Terminal,
        //     instr: None,
        //     term: None,
        // };
        // let mut v = vec![pp1, pp_term, pp11, pp2];
        // v.sort();
        // for pp in &v {
        //     print!("{}, ", pp.to_string());
        // }
        // println!();

        // "bb8.term".to_string(),
        // "bb7.term".to_string(),
        // "bb13.3".to_string(),
        // "bb17.term".to_string(),
        // "bb13.0".to_string(),
        // "bb12.2".to_string(),
        // "bb12.1".to_string(),
        // "bb6.2".to_string(),
        // "entry.12".to_string(),
        // "bb12.term".to_string(),
        // "bb6.1".to_string(),
        // "bb13.2".to_string(),
        // "bb12.0".to_string(),
        // "entry.6".to_string(),
        // "entry.term".to_string(),
        // "bb17.2".to_string(),
        // "entry.4".to_string(),
        // "entry.7".to_string(),
        // "entry.11".to_string(),
        // "bb12.4".to_string(),
        // "entry.5".to_string(),
        // "entry.13".to_string(),
        // "bb5.term".to_string(),
        // "entry.2".to_string(),
        // "entry.3".to_string(),
        // "bb17.0".to_string(),
        // "bb12.3".to_string(),
        // "bb6.3".to_string(),
        // "entry.9".to_string(),
        // "bb13.4".to_string(),
        // "bb13.term".to_string(),
        // "bb6.term".to_string(),
        // "bb4.term".to_string(),
        // "bb11.term".to_string(),
        let mut pps = vec![
            ProgramPoint {
                block: "bb8".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb7".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb13".to_string(),
                location: Location::Instruction(3),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb17".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb13".to_string(),
                location: Location::Instruction(0),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb12".to_string(),
                location: Location::Instruction(2),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb12".to_string(),
                location: Location::Instruction(1),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb6".to_string(),
                location: Location::Instruction(2),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(12),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb12".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb6".to_string(),
                location: Location::Instruction(1),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb13".to_string(),
                location: Location::Instruction(2),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb12".to_string(),
                location: Location::Instruction(0),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(6),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb17".to_string(),
                location: Location::Instruction(2),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(4),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(7),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(11),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb12".to_string(),
                location: Location::Instruction(4),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(5),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(13),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb5".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(2),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(3),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb17".to_string(),
                location: Location::Instruction(0),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb12".to_string(),
                location: Location::Instruction(3),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb6".to_string(),
                location: Location::Instruction(3),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "entry".to_string(),
                location: Location::Instruction(9),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb13".to_string(),
                location: Location::Instruction(4),
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb13".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb6".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb4".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
            ProgramPoint {
                block: "bb11".to_string(),
                location: Location::Terminal,
                instr: None,
                term: None,
            },
        ];
        pps.sort();
        for pp in &pps {
            print!("{}, ", pp.to_string());
        }
        println!();

        let res = vec![
            "bb11.term".to_string(),
            "bb12.0".to_string(),
            "bb12.1".to_string(),
            "bb12.2".to_string(),
            "bb12.3".to_string(),
            "bb12.4".to_string(),
            "bb12.term".to_string(),
            "bb13.0".to_string(),
            "bb13.2".to_string(),
            "bb13.3".to_string(),
            "bb13.4".to_string(),
            "bb13.term".to_string(),
            "bb17.0".to_string(),
            "bb17.2".to_string(),
            "bb17.term".to_string(),
            "bb4.term".to_string(),
            "bb5.term".to_string(),
            "bb6.1".to_string(),
            "bb6.2".to_string(),
            "bb6.3".to_string(),
            "bb6.term".to_string(),
            "bb7.term".to_string(),
            "bb8.term".to_string(),
            "entry.2".to_string(),
            "entry.3".to_string(),
            "entry.4".to_string(),
            "entry.5".to_string(),
            "entry.6".to_string(),
            "entry.7".to_string(),
            "entry.9".to_string(),
            "entry.11".to_string(),
            "entry.12".to_string(),
            "entry.13".to_string(),
            "entry.term".to_string(),
        ];
        assert_eq!(pps.iter().map(|pp| pp.to_string()).collect::<Vec<String>>(), res);
        
    }
}
