# UCSB CS260 (Program Analysis) Course Projects

[lir-description.md](./lir-description.md): Low-level Intermediate Representation (LIR) format description.


## Lab 0: Low-level Intermediate Representation parsing


Herein we first use `lir2json` program provided by the instructor that converts the LIR to JSON format. Then we use `serde_json` in Rust to parse the JSON files.

In the folder `./assign-0/`, run

```shell
cargo run
```

to test all parsed programs and compare the statistics info with the expected results.


*Comment: When you obverse the series of .json files, you will find its is quite like Rust-style struct definitions with *serde* serialization and deserialization.*


Run `./prase_json <json file>` to test the functionality of parsing a specific .json file. `parse_json` is compiled through my own `cargo build`. *The Professor has provide a program named `lir2json` to convert .lir files to .json files.*


```shell
./parse_json ./tests/test.1.1.lir.json
```

Run `./parse_lir <lir file>` to test the functionality of parsing a specific .lir file. `parse_lir` is a reference parsing program provide by the Professor.

```shell
./parse_lir ./tests/test.1.1.lir
```


`./assign-0/tests/` includes some benchmark programs.

## Lab 1: Intraprocedure Dataflow Analysis (constant analysis & interval analysis)

See `./assign-1/Makefile` to run commands.

- `make` to compile Rust programs into two executables `constants_analysis` and `intervals_analysis`.
- `make constants_analysis` to only compile the constant analysis program.
- `make intervals_analysis` to only compile the interval analysis program.


`build-analyses.sh`, `run-constants-analysis.sh`, and `run-intervals-analysis.sh` are required scripts to be submitted on GradeScope.

`./assign-1/examples/` includes some benchmark programs.

Run `constants_analysis <json file> <func name>` to test a single case of constant analysis.

Run `intervals_analysis <json file> <func name>` to test a single case of interval analysis.
