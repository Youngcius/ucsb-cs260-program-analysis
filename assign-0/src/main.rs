// parse LIR files, get stats, compare

mod lir;
mod stats;
use lir::Program;
use stats::Stats;
use std::fs;

fn main() {
    let paths = fs::read_dir("./tests").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".lir") {
            let fname = path_str.replace(".lir", "");
            let json_fname = format!("{}.lir.json", fname);
            let stats_fname = format!("{}.stats", fname);
            println!("Testing parsing {} ...", fname);

            let stats = Stats::read_stats(&stats_fname);
            let program = Program::parse_json(&json_fname);
            assert_eq!(stats, program.get_stats());
        }
    }
}
