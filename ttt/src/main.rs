use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_variables)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Deserialize, Serialize, Debug)]
// #[serde(untagged)]
enum Type {
    Int,
    Struct(String),
}

#[derive(Deserialize, Serialize, Debug)]
enum FunctionType {
    Int,
    Struct(String),
}

impl FunctionType {
    fn new() -> FunctionType {
        FunctionType::Int
    }
    fn do_something(&self) {
        match self {
            FunctionType::Int => {
                println!("do something for int");
            }
            FunctionType::Struct(s) => {
                println!("do something for struct: {}", s);
            }
        }
    }
}

// "Function": {
//     "ret_ty": "Int",
//     "param_ty": []
// }

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Student {
    name: Option<String>,
    score: Option<i32>,
}

impl Student {
    fn add_score_with_10(&mut self) {
        self.score = Some(self.score.unwrap() + 10);
    }

    fn print_name(&self) {
        match self.name {
            Some(ref name) => println!("!!!name: {}", name),
            _ => println!("!!!name: None???"),
        }
    }
}

fn add_func(a: &i32, b: &i32) -> i32 {
    if (*a) > 10 {
        return 100;
    }
    a + b
}

fn main() {
    let ty = Type::Int;
    let ty1 = Type::Struct("xxx".to_string());

    println!("println: ty... {:?}", &ty);
    println!("println: ty1... {:?}", &ty1);

    println!(
        "json serialization: ty... {}",
        serde_json::to_string(&ty).unwrap()
    );
    println!(
        "json serialization: ty1... {}",
        serde_json::to_string(&ty1).unwrap()
    );

    let ft = FunctionType::new();

    ft.do_something();

    let s = Student {
        name: Some("zhangsan".to_string()),
        score: Some(100),
    };
    println!("s: {:?}", &s); // { name: Some("zhangsan"), score: Some(100) }
                             // println!("name of s: {}", s.name.unwrap()); // zhangsan
    println!("score of s: {:?}", s.score); // Some(100)
    s.print_name();

    let v = Some(9);
    let x = v.unwrap() + 1;
    println!("x: {}", x); // 10

    let c1 = 10;
    let c2 = 20;
    let b1 = &c1;
    let b2 = &c2;
    let d = add_func(b1, b2);
    println!("d: {}", d); // 30

    println!("True as i32: {}", true as i32); // 1
    println!("False as i32: {}", false as i32); // 0

    let s1 = Student {
        name: Some("zhangsan".to_string()),
        score: Some(100),
    };
    let mut s2 = Student {
        name: Some("zhangsan".to_string()),
        score: Some(100),
    };
    println!("s1 =? s2: {}", s1 == s2);
    println!("s1 =? s1: {}", s1 == s1);

    s2.score = Some(1100);
    println!("{:?}", s2); // Student { name: Some("zhangsan"), score: Some(1100) }

    s2.add_score_with_10();

    let mut s3 = &mut s2;
    s3.add_score_with_10();

    println!("{:?}", s3); // Student { name: Some("zhangsan"), score: Some(1120) }
    println!("{:?}", s2); // Student { name: Some("zhangsan"), score: Some(1120) }

    let mut dict1 = HashMap::new();
    let mut dict2 = HashMap::new();
    dict2.insert("name", "zhangsan");
    dict2.insert("score", "100");
    dict1.insert("score", "100");
    dict1.insert("name", "zhangsan");
    println!("{:?}", dict1); // {"name": "zhangsan", "score": "100"}
    println!("{:?}", dict2); // {"name": "zhangsan", "score": "100"}
    println!("{:?}", dict1 == dict2); // true

    #[cfg(debug_assertions)]
    {
        println!("debug mode");
    }

    println!("normal mode");
    #[cfg(debug_assertions)]
    println!("debug mode");
    println!("normal mode");

    println!("i32::MAX: {}", i32::MAX); // 2147483647
    println!("i32::MIN: {}", i32::MIN); // -2147483648
                                        // println!("i32::MAX + 1: {}", i32::MAX + 1); // -2147483648
    println!("i32::MAX / 2 + 1: {}", i32::MAX / 2 + 1); // 1073741824
    println!("i32::MIN / 2: {}", i32::MIN / 2); // -1073741824

    let inf = f64::INFINITY;
    let ninf = f64::NEG_INFINITY;

    println!("inf: {}", inf); // inf
    println!("ninf: {}", ninf); // -inf

    println!("inf + 1: {}", inf + 1.0); // inf

    println!("inf > 1000000: {}", inf > 1000000.0); // true

    println!("0 / inf: {}", 0.0 / inf); // 0

    println!("inf / 0: {}", inf / 0.0); // inf

    println!("inf / inf: {}", inf / inf); // nan

    println!("inf + ninf: {}", inf + ninf); // nan

    // println!("0 / 0: {}", 0 / 0); // thread 'main' panicked at 'attempt to divide by zero', src/main.rs:123:5
    // println!("10 / 0: {}", 10 / 0); // thread 'main' panicked at 'attempt to divide by zero', src/main.rs:123:5

    println!();
    println!("---------------------------------------------------------------");
    println!();

    let n0 = Number::Integer(0);
    let n1m = Number::Integer(-1);
    let n10 = Number::Integer(10);
    let n20 = Number::Integer(20);
    let n11m = Number::Integer(-11);
    let n15m = Number::Integer(-15);
    let inf = Number::Infinity;
    let ninf = Number::NInfinity;

    let mut v = vec![n0.clone(), n1m.clone(), n10.clone(), n20.clone(), n11m.clone(), n15m.clone(), inf.clone(), ninf.clone()];
    v.sort();

    println!("v: {:?}", v); // [NegInf, -15, -11, -1, 0, 10, 20, PosInf]


    // test add operation of Number
    println!();
    println!("n10 + n20: {}", n10.clone() + n20.clone()); // 30
    println!("n10 + n11m: {}", n10.clone() + n11m.clone()); // -1
    println!("n10 + inf: {}", n10.clone() + inf.clone()); // PosInf
    println!("n10 + ninf: {}", n10.clone() + ninf.clone()); // NegInf
    // println!("inf + ninf: {}", inf.clone() + ninf.clone()); // PosInf

    // test sub operation of Number
    println!();
    println!("n10 - n20: {}", n10.clone() - n20.clone()); // -10
    println!("n10 - n11m: {}", n10.clone() - n11m.clone()); // 21
    println!("n10 - inf: {}", n10.clone() - inf.clone()); // NegInf
    println!("n10 - ninf: {}", n10.clone() - ninf.clone()); // PosInf
    println!("inf - ninf: {}", inf.clone() - ninf.clone()); // PosInf
    println!("ninf - inf: {}", ninf.clone() - inf.clone()); // NegInf

    // test mul operation of Number
    println!();
    println!("n10 * n20: {}", n10.clone() * n20.clone()); // 200
    println!("n10 * n11m: {}", n10.clone() * n11m.clone()); // -110
    println!("n10 * inf: {}", n10.clone() * inf.clone()); // PosInf
    println!("n10 * ninf: {}", n10.clone() * ninf.clone()); // NegInf
    println!("inf * ninf: {}", inf.clone() * ninf.clone()); // NegInf
    println!("ninf * inf: {}", ninf.clone() * inf.clone()); // NegInf
    println!("n0 * inf: {}", n0.clone() * inf.clone()); // 0
    println!("n0 * ninf: {}", n0.clone() * ninf.clone()); // 0

    // test div operation of Number
    println!();
    println!("n10 / n20: {}", n10.clone() / n20.clone()); // 0
    println!("n10 / n11m: {}", n10.clone() / n11m.clone()); // 0
    println!("n10 / inf: {}", n10.clone() / inf.clone()); // 0
    println!("n10 / ninf: {}", n10.clone() / ninf.clone()); // 0
    println!("inf / n10: {}", inf.clone() / n10.clone()); // PosInf
    println!("ninf / n10: {}", ninf.clone() / n10.clone()); // NegInf
    // println!("inf / inf: {}", inf.clone() / inf.clone()); // PosInf
    // println!("ninf / ninf: {}", ninf.clone() / ninf.clone()); // PosInf
    println!("n0 / inf: {}", n0.clone() / inf.clone()); // 0
    println!("n0 / ninf: {}", n0.clone() / ninf.clone()); // 0
    // println!("inf / 0: {}", inf.clone() / n0.clone()); // 0

    // test cmp operation
    println!();
    println!("n10 == n20: {}", n10 == n20); // false
    println!("n10 == n10: {}", n10 == n10); // true
    println!("n10 != n20: {}", n10 != n20); // true
    println!("n10 < inf: {}", n10 < inf); // true
    println!("n10 > ninf: {}", n10 > ninf); // true
    println!("ninf < inf: {}", ninf <= inf); // true
    println!("inf >= ninf: {}", inf >= ninf); // true


    println!("inf / (-1): {}", inf / n1m.clone()); // NegInf
    println!("ninf / (-1): {}", ninf / n1m.clone()); // PosInf



}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    NInfinity,
    Integer(i32),
    Infinity,
}

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
