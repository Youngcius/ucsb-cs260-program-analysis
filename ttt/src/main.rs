#[allow(dead_code)]
#[allow(unused_variables)]
use serde::{Deserialize, Serialize};
use serde_json;

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
    let s2 = Student {
        name: Some("zhangsan".to_string()),
        score: Some(100),
    };
    println!("s1 =? s2: {}", s1 == s2);
    println!("s1 =? s1: {}", s1 == s1);
}
