// parse LIR files, get stats, compare

use cs260::lir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --bin parse_json <json_file>");
        std::process::exit(1);
    }
    let json_fname = &args[1];

    let prog = lir::Program::parse_json(json_fname);

    println!("{:#?}", prog.get_stats());
}
