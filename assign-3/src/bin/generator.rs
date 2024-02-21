use cs260::abs;
use cs260::lir;
use cs260::utils;

fn main() {
    // accept command line arguments (./generator <json_file>)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --bin generator <json_file>");
        std::process::exit(1);
    }
    let json_fname = &args[1];

    let prog = lir::Program::parse_json(&json_fname);
    let mut analyzer = abs::execution::ConstraintGenerator::new(prog);
    // analyzer.execute();
    // utils::display_ctrl_solution(&analyzer.solution);
}
