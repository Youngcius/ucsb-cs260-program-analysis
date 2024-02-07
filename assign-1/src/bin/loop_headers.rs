use cs260::cfg;
use cs260::lir;

fn main() {
    // accept command line arguments (./constants_analysis <json_file> <func_name>)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run --bin constants_analysis <json_file> <func_name>");
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
    let cfg = cfg::ControlFlowGraph::from_function(&prog, func_name);
    let loop_headers = cfg.get_loop_headers();
    println!("Loop headers: {:?}", loop_headers);
}
