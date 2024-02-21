use cs260::lir;
use std::collections::{HashMap, HashSet};
use std::fs;


fn main() {
    let paths = fs::read_dir("./tests/json").unwrap();
    let mut prog_instrs: HashMap<String, HashSet<String>> = HashMap::new();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        let prog = lir::Program::parse_json(&path_str);
        let func = prog.functions.get("test").unwrap();
        for (_, block) in &func.body {
            for instr in &block.insts {
                prog_instrs
                    .entry(path_str.to_string())
                    .or_insert(HashSet::new())
                    .insert(instr.get_name());
            }
            prog_instrs
                .entry(path_str.to_string())
                .or_insert(HashSet::new())
                .insert(block.term.get_name());
        }
    }
    println!("{:#?}", prog_instrs);
}
