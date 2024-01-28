# cs260 w24 assignment 0

__This is not a graded assignment__ and you will not turn it in. It is intended purely to help you prepare the necessary infrastructure for the real assignments.

The goal is to read in a program in LIR format (as described in the accompanying file `lir-description.md`) from a file and store it in a data structure that allows you to easily iterate over functions, basic blocks, and instructions. This is something that you will need to do for all the upcoming assignments in this class. The LIR format is intended to be very easy to parse; the only context-free part is types (see the grammar in `lir-description.md`) and those are still designed to be trivial to parse.

Optionally, if it's easier for you, the LIR programs are also provided in JSON format. These are the exact same programs, just pre-parsed and then serialized as JSON objects. You can choose to __either__ parse the LIR format __or__ read in the JSON format; you __don't__ need to do both.

A set of LIR programs is contained in the accompanying file `tests.zip`. To help determine whether you read in the program correctly, for each program file `<name>.lir` there is a associated file `<name>.stats` that prints out a set of statistics---you can compute the same statistics from your own data structure and compare to make sure they are the same. To be clear, the statistics don't matter themselves, they are just a way to help you determine if you can read in and iterate over programs correctly.

The statistics are:

- Number of fields across all struct types
- Number of functions that return a value
- Number of function parameters
- Number of local variables
- Number of basic blocks
- Number of instructions
- Number of terminals
- Number of locals and globals with int type
- Number of locals and globals with struct type
- Number of locals and globals with pointer to int type
- Number of locals and globals with pointer to struct type
- Number of locals and globals with pointer to function type
- Number of locals and globals with pointer to pointer type

**Implementation**:

Herein I use Rust to define reasonable `struct`s and use the *serde* library to parse .json files.

Run `cargo run` to verify this data structure definitions and parsing functionalities. 

*Comment: When you obverse the series of .json files, you will find its is quite like Rust-style struct definitions with *serde* serialization and deserialization.*


Run `./prase_json <json file>` to test the functionality of parsing a specific .json file. `parse_json` is compiled through my own `cargo build`. *The Professor has provide a program named `lir2json` to convert .lir files to .json files.*


```shell
./parse_json ./tests/test.1.1.lir.json
```

Run `./parse_lir <lir file>` to test the functionality of parsing a specific .lir file. `parse_lir` is a reference parsing program provide by the Professor.

```shell
./parse_lir ./tests/test.1.1.lir
```


**References**:

[PyParsing tutorial](https://zhuanlan.zhihu.com/p/259638397)

[Nom Rust](https://llever.com/gentle-intro/nom-intro.zh.html)

[pest.rs](https://ohmyweekly.github.io/notes/2021-01-20-pest-grammars/)

[Rust 开发编译器速成（一）：计算解释器](https://www.less-bug.com/posts/rust-development-compiler-crash-1-calc-interpreter/)

[Serde Json](https://juejin.cn/post/7220463381493022757)

