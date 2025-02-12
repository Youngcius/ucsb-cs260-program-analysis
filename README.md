# UCSB CS260 (Program Analysis) Course Projects

This repository contains a series of course projects of UCSB CS260 (Program Analysis) in Winter 2024, about implementations of typical static program analysis techniques such as interval analysis, reaching definition analysis and set constraint analysis.

Source codes for these "analyzers" in these projects are written in Rust. Input are .json files and output are text files.

[lir-description.md](./lir-description.md): Low-level Intermediate Representation (LIR) format description.


## Lab 0: Low-level Intermediate Representation parsing

*See [lab0's readme file](./assign-0/readme.md)* for details of the project description.

Herein we first use `lir2json` program provided by the instructor that converts the LIR to JSON format. Then we use `serde_json` in Rust to parse the JSON files.

In the folder `./assign-0/`, run

```shell
make
```

to compile the project and generate two executable files `cs260` and `parse_json`.

Then run

```shell
make test
```

to test all parsed programs and compare the statistics info with the expected results.

*Comment: When you obverse the series of .json files, you will find its is quite like Rust-style struct definitions with *serde* serialization and deserialization.*

Also, you can run `./prase_json <json file>` to test the parsing functionality on a specific .json file. `parse_json` is compiled through my own `cargo build`. *The Professor has provide a program named `lir2json` to convert .lir files to .json files.*


Run `./prase_json <json file>` to test the functionality of parsing a specific .json file.


```shell
./parse_json ./tests/test.1.1.lir.json
```

Run `./parse_lir <lir file>` to test the functionality of parsing a specific .lir file. `parse_lir` is a reference parsing program provide by the Professor.

```shell
./parse_lir ./tests/test.1.1.lir
```


`./assign-0/tests/` includes some benchmark programs.

## Lab 1: Intraprocedure Dataflow Analysis (constant analysis & interval analysis)

*See [lab1's readme file](./assign-1/readme.md)* for details of the project description.

`constants_analysis` and `intervals_analysis` are two main executable files compiled by this *assign-1* project.

Run `constants_analysis <json file> <func name>` to test a single case of constant analysis.

Run `intervals_analysis <json file> <func name>` to test a single case of interval analysis.

See `./assign-1/Makefile` for detailed commands.

- `make` to compile Rust programs into two executables `constants_analysis` and `intervals_analysis`.
- `make debug` to compile Rust programs in *debug* mode so that the output program will print debugging info when executing.
- `make constants_analysis` to only compile the constant analysis program.
- `make intervals_analysis` to only compile the interval analysis program.
- `make package` to copy necessary source files into a single folder and package them into a *.zip* file to be submitted on GradeScope. (`build-analyses.sh`, `run-constants-analysis.sh`, and `run-intervals-analysis.sh` are required scripts to be submitted on GradeScope.)

For benchmarking:

- `./assign-1/examples/` includes some simple benchmark programs. Use `. bench-example-const.sh` and `. bench-example-inter.sh` to test all benchmark LIR programs within it by using the standard `constants` analyzer provided by the instructor and our `constants_analysis` analyzer and further compare their results.
- `./assign-1/demos` includes some complicated benchmark programs (most of them are from GradeScope). Use `. test-demo-const.sh` and `. test-demo-inter.sh` to test all benchmark LIR programs within it and generated *.out* files will be dumped in `./assign-1/demos` as well. Then run scripts `diff-demo-const.sh` and `diff-demo-inter.sh` to compare all outputted results generated by the standard analyzer and our analyzer.
- Also, `make test`, `make test_constants_analysis` and `make test_intervals_analysis`integrate corresponding testing functionalities.

## Lab 2: Deaching definitions analysis & Control dependence analysis

*See [lab2's readme file](./assign-2/readme.md)* for details of the project description.

`rdef_analysis` and `ctrl_analysis` are two main executable files compiled by the *assign-2* project.

Run `rdef_analysis <json file> <fun name>` to test a single case of reaching definitions analysis.

Run `ctrl_analysos <json file>` to test a single case of control dependence analysis.

See `./assign-2/Makefile` for detailed commands.

- `make` to compile Rust programs into two executables `rdef_analysis` and `ctrl_analysis`.
- `make debug` to compile Rust programs in *debug* mode so that the ouput rpogram will print debugging info when executing.
- `make rdef_analysis` to only compile the reaching definitions analysis program.
- `make ctrl_analysis` to only compile the control dependence analysis program.
- `make package` to copy necessary source files into a single folder and package them into a *.zip* file to be submitted on GradeScope.  (`build-analyses.sh`, `run-constants-analysis.sh`, and `run-intervals-analysis.sh` are required scripts to be submitted on GradeScope.)

For benchmarking:

- `./assign-2/tests` includes some complicated benchmark programs (most of them are from GradeScope). Use `. test-tests-rdef.sh` and `. test-tests-ctrl.sh` to test all benchmark LIR programs within it and generated *.out* files will be dumped in `./assign-2/tests` as well. Then run scripts `diff-tests-rdef.sh` and `diff-tests-ctrl.sh` to compare all outputted results generated by the standard analyzer and our analyzer.
- Also, `make test`, `make test_reaching_de` and `make test_intervals_analysis`integrate corresponding testing functionalities.

