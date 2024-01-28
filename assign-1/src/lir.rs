/*
Low-level intermediate representation (LIR) for program analysis.
*/
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

// use super::abs::execution::AbstractExecution;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    // it could be as parameter, local variable, or global variable
    pub name: String,
    pub typ: Type,
    pub scope: Option<String>,
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

impl Program {
    pub fn new() -> Program {
        Program {
            structs: HashMap::new(),
            globals: Vec::new(),
            functions: HashMap::new(),
            externs: HashMap::new(),
        }
    }

    pub fn get_all_globals(&self) -> &Vec<Variable> {
        &self.globals
    }

    pub fn get_all_parameters(&self) -> Vec<&Variable> {
        let mut params = Vec::new();
        for func in self.functions.values() {
            for param in &func.params {
                params.push(param);
            }
        }
        params
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
