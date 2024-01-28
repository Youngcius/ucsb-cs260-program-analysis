// parse LIR files, get stats, compare
#![allow(dead_code)]
#![allow(unused_imports)]

// mod lir;
// mod stats;




use cs260::lir::Program;

use std::fs;

fn main() {
    // accept command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run --bin int_constants <json_file> <func_name>");
        std::process::exit(1);
    }
    let json_fname = &args[1];
    let func_name = &args[2];

    println!("json_fname: {}", json_fname);
    println!("func_name: {}", func_name);

    let prog = Program::parse_json(&json_fname);

    // let paths = fs::read_dir("./tests").unwrap();

    // for path in paths {
    //     let path = path.unwrap().path();
    //     let path_str = path.to_str().unwrap();
    //     if path_str.ends_with(".lir") {
    //         let fname = path_str.replace(".lir", "");
    //         let json_fname = format!("{}.lir.json", fname);
    //         let stats_fname = format!("{}.stats", fname);
    //         println!("Testing parsing {} ...", fname);

    //         let stats = Stats::read_stats(&stats_fname);
    //         let program = Program::parse_json(&json_fname);
    //         assert_eq!(stats, program.get_stats());
    //     }
    // }
    // println!("This is constants analysis")
}
