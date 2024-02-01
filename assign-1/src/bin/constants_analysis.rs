#![allow(dead_code)]
#![allow(unused_imports)]

use cs260::abs;
use cs260::abs::execution::AbstractExecution;
use cs260::lir;
use cs260::store;
use cs260::utils;
use std::fs;

fn main() {
    // accept command line arguments (./constants_analysis <json_file> <func_name>)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run --bin constants_analysis <json_file> <func_name>");
        std::process::exit(1);
    }
    let json_fname = &args[1];
    let func_name = &args[2];

    println!("json_fname: {}", json_fname);
    println!("func_name: {}", func_name);

    let prog = lir::Program::parse_json(&json_fname);
    let mut analyzer = abs::execution::ConstantAnalyzer::new(prog, &func_name); // TODO: "mut" is necessary?
    analyzer.mfp();
    utils::display_bb2store(&analyzer.bb2store);
    // println!("length of bb2store: {}", analyzer.bb2store.len());

    // TODO: dump to temp files, compare with results from the analyzer provided by Professor
}
