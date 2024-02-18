use cs260::cfg;
use cs260::lir;

fn main() {
    // accept command line arguments (./constants_analysis <json_file> <func_name>)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run --bin gene_graphml <json_file> <func_name> <output_file>");
        std::process::exit(1);
    }
    let json_fname = &args[1];
    let func_name = &args[2];
    let mut filename = args[3].clone();
    if !filename.ends_with(".graphml") {
        filename.push_str(".graphml");
    }
    
    #[cfg(debug_assertions)]
    {
        println!("json_fname: {}", json_fname);
        println!("func_name: {}", func_name);
    }

    let prog = lir::Program::parse_json(&json_fname);
    let cfg = cfg::ControlFlowGraph::from_function(&prog, &func_name);
    let _ = cfg.to_graphml_file(&filename);
}
