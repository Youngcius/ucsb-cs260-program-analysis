use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub num_fields: u32,
    pub num_functions: u32,
    pub num_func_params: u32,
    pub num_locals: u32,
    pub num_basic_blocks: u32,
    pub num_instructions: u32,
    pub num_terminals: u32,
    pub num_ints: u32,        // number of locals and globals with int type
    pub num_structs: u32,     // number of locals and globals with struct type
    pub num_ptr_ints: u32,    // number of locals and globals with pointer to int type
    pub num_ptr_structs: u32, // number of locals and globals with pointer to struct type
    pub num_ptr_funcs: u32,   // number of locals and globals with pointer to function type
    pub num_ptr_ptrs: u32,    // number of locals and globals with pointer to pointer type
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            num_fields: 0,
            num_functions: 0,
            num_func_params: 0,
            num_locals: 0,
            num_basic_blocks: 0,
            num_instructions: 0,
            num_terminals: 0,
            num_ints: 0,
            num_structs: 0,
            num_ptr_ints: 0,
            num_ptr_structs: 0,
            num_ptr_funcs: 0,
            num_ptr_ptrs: 0,
        }
    }

    pub fn read_stats(fname: &str) -> Stats {
        /*
        Read the stats file and return a Stats struct.
        The content of stats file named fname (a text file) is as follows:

            Number of fields across all struct types: 2
            Number of functions that return a value: 4
            Number of function parameters: 8
            Number of local variables: 1039
            Number of basic blocks: 391
            Number of instructions: 1234
            Number of terminals: 391
            Number of locals and globals with int type: 344
            Number of locals and globals with struct type: 21
            Number of locals and globals with pointer to int type: 195
            Number of locals and globals with pointer to struct type: 83
            Number of locals and globals with pointer to function type: 57
            Number of locals and globals with pointer to pointer type: 346

         */
        let file = File::open(fname).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut stats = Stats {
            num_fields: 0,
            num_functions: 0,
            num_func_params: 0,
            num_locals: 0,
            num_basic_blocks: 0,
            num_instructions: 0,
            num_terminals: 0,
            num_ints: 0,
            num_structs: 0,
            num_ptr_ints: 0,
            num_ptr_structs: 0,
            num_ptr_funcs: 0,
            num_ptr_ptrs: 0,
        };

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(": ").collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "Number of fields across all struct types" => {
                            stats.num_fields = parts[1].parse().unwrap();
                        }
                        "Number of functions that return a value" => {
                            stats.num_functions = parts[1].parse().unwrap();
                        }
                        "Number of function parameters" => {
                            stats.num_func_params = parts[1].parse().unwrap();
                        }
                        "Number of local variables" => {
                            stats.num_locals = parts[1].parse().unwrap();
                        }
                        "Number of basic blocks" => {
                            stats.num_basic_blocks = parts[1].parse().unwrap();
                        }
                        "Number of instructions" => {
                            stats.num_instructions = parts[1].parse().unwrap();
                        }
                        "Number of terminals" => {
                            stats.num_terminals = parts[1].parse().unwrap();
                        }
                        "Number of locals and globals with int type" => {
                            stats.num_ints = parts[1].parse().unwrap();
                        }
                        "Number of locals and globals with struct type" => {
                            stats.num_structs = parts[1].parse().unwrap();
                        }
                        "Number of locals and globals with pointer to int type" => {
                            stats.num_ptr_ints = parts[1].parse().unwrap();
                        }
                        "Number of locals and globals with pointer to struct type" => {
                            stats.num_ptr_structs = parts[1].parse().unwrap();
                        }
                        "Number of locals and globals with pointer to function type" => {
                            stats.num_ptr_funcs = parts[1].parse().unwrap();
                        }
                        "Number of locals and globals with pointer to pointer type" => {
                            stats.num_ptr_ptrs = parts[1].parse().unwrap();
                        }
                        _ => {}
                    }
                }
            }
        }
        stats
    }
}
