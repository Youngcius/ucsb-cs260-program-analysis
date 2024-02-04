/*
Statistics all function names of all programs in the examples/json directory
*/
use cs260::lir;
use std::collections::HashMap;
use std::fs;

fn main() {
    let paths = fs::read_dir("./examples/json").unwrap();
    let mut prog_funcs: HashMap<String, Vec<String>> = HashMap::new();

    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        assert!(path_str.ends_with(".json"));
        let prog = lir::Program::parse_json(&path_str);
        let func_names: Vec<String> = prog.functions.keys().cloned().collect();
        let prog_name = path_str.replace(".json", "");
        let prog_name: Vec<&str> = prog_name.split("/").collect();
        let prog_name = prog_name[prog_name.len() - 1];
        prog_funcs.insert(prog_name.to_string(), func_names);
    }

    let prog_funcs_json = serde_json::to_string_pretty(&prog_funcs).unwrap();
    fs::write("./examples/prog_funcs.json", prog_funcs_json).unwrap();
}
