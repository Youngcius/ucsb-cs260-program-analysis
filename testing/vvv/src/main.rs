// use user_info::user::User;
// use crate::user_info::user::User;
#[macro_use]
use vvv::user_info;
use vvv::user_info::user::User;
use vvv::user_info::user;

use vvv::hashset;

use std::collections::HashSet;




fn main() {
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));
    

    let hash_set = hashset!{1, 2, 3, 4, 5};

    println!("hash_set: {:?}", hash_set);
}
