// use user_info::user::User;
// use crate::user_info::user::User;
use std::collections::HashMap;
use std::collections::HashSet;
use vvv::hashset;
use vvv::user_info;
use vvv::user_info::user;
use vvv::user_info::user::User;

fn main() {
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));

    let hash_set = hashset! {1, 2, 3, 4, 5};

    println!("hash_set: {:?}", hash_set);

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    let pp = "key";
    let var = "value";

    map.entry(pp.to_string())
        .or_insert(HashSet::new())
        .insert(var.to_string());

    println!("{:?}", map);

    map.clear();

    let vec = vec!["a", "b", "c", "d", "e"];
    println!("{:?}", map);

    for v in vec.iter() {
        map.entry(pp.to_string())
            .or_insert(HashSet::new())
            .insert(v.to_string());
    }

    println!("{:?}", map);
    println!("vec: {:?}", vec);

}
