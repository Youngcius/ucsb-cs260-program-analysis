use cs260::abs;
use cs260::lir;
use cs260::utils;

fn main() {
    // accept command line arguments (./ctrl_analysis <json_file> <func_name>)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run --bin ctrl_analysis <json_file> <func_name>");
        std::process::exit(1);
    }
    let json_fname = &args[1];
    let func_name = &args[2];

    #[cfg(debug_assertions)]
    {
        println!("json_fname: {}", json_fname);
        println!("func_name: {}", func_name);
    }

    let prog = lir::Program::parse_json(&json_fname);
    let mut analyzer = abs::execution::ControlDependenceAnalyzer::new(prog, &func_name);
    analyzer.execute();
    utils::display_ctrl_solution(&analyzer.solution);
}
