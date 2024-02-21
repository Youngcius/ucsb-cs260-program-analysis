use cs260::abs;
use cs260::lir;
use cs260::utils;

fn main() {
    // accept command line arguments (./solver <constraints_file>)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --bin solver <constraints_file>");
        std::process::exit(1);
    }
    let constraints_file = &args[1];
    // let constraint_set = ...
    let mut analyzer = abs::execution::ConstraintSolver::new(constraint_set);
    // analyzer.execute();
    // utils::display_ctrl_solution(&analyzer.solution);
}
