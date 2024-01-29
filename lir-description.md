# NOTES

- you can assume that all input programs used for the assignments are syntactically correct and valid (i.e., it meets all validity requirements outlined below).

- if you have a question about what is a valid program, you can come up with some examples and use the provided solution on CSIL (see `~benh/260/`) to test them.

# LIR description

- a LIR program is:

    - a set of struct types
    - a set of global variables
    - a set of extern function declarations
    - a set of function definitions

- a type is one of:

    - `int` (an integer)
    - a struct type (identified by its name)
    - a function type
    - a pointer to a type

- a struct type is a name and a non-empty list of field names and types.

    - these are just like C-style structs.

- a global variable is a name and a type.

- an extern function declaration is a name and a function type.

    - an extern function is one that is defined externally to the current program, i.e., we know its name and type signature and can call it, but we don't have access to its source code.

- a function definition is:

    - a name
    - an ordered list of parameters (with names and types)
    - an optional return type
    - a set of local variables (with names and types)
    - a set of basic blocks

- a basic block is a label and an ordered list of instructions, ending in a single terminal.

- an instruction is one of:

    - address-of: `x = $addrof y` assigns the address of `y` to `x`.

    - memory allocation: `x = $alloc 10 [_a1]` allocates 10 items (what type of items depends on the type of pointer variable `x`) and assigns the newly allocated address to `x`. the bracketed identifier (`_a1` in this example) is a convenient label for the allocation that we will use during some of our analyses.

    - integer arithmetic: `x = $arith add y 2` adds 2 to the value of `y` and assigns the result to `x`; similarly for the other arithmetic operators `sub`, `mul`, `div`.
    
    - integer and pointer comparison `x = $cmp eq y 2` compares `y` to 2 and assigns 0 (false) or 1 (true) to `x`; similarly for the other relational operators `neq`, `lt`, `lte`, `gt`, `gte`.

    - copy: `x = $copy y` copies the value of `y` to `x`.

    - get-element-pointer: `x = $gep y 10` takes `y` (which is a pointer to an array of elements) and assigns to `x` the address of the 10th element of the array. this is the only way to do pointer arithmetic.

    - get-field-pointer: `x = $gfp y foo` takes `y` (which is a pointer to a struct) and assigns to `x` the address of the `foo` field of the struct. the only way to access fields of a struct is via a pointer.

    - load from memory: `x = $load y` loads the value pointed to by `y` and assigns it to `x`.

    - store to memory: `$store x y` stores the value of `y` to the location pointed to by `x`.

    - call external function: `x = $call_ext foo(10)` calls the extern function `foo` with argument 10 and assigns the returned value to `x`. if the extern function does not return a value then there is no assignment, e.g., `$call_ext foo(10)`.

- a terminal signals the end of a basic block and is one of:

    - conditional branch: `$branch x bb1 bb2` will either jump to basic block `bb1` (if the value of `x` is non-zero) or `bb2` (if the value of `x` is 0).

    - unconditional jump: `$jump bb1` will always jump to basic block `bb1`.

    - return from function: `$ret x` will return from the current function with the value of `x`. if the current function does not return a value then it would just be `$ret`.

    - direct function call: `x = $call_dir foo(10) then bb1` calls internally-defined function `foo` passing the argument 10, assigns the returned value to `x`, then jumps to basic block `bb1`. if `foo` does not return a value then there is no assignment, e.g., `$call_dir foo(10) then bb1`.

    - indirect function call: `x = $call_idr fp(10) then bb1` calls an internally-defined function (which one depends on the value of the function pointer variable `fp`) passing the argument 10, assigns the returned value to `x`, then jumps to basic block `bb1`. if the called function does not return a value then there is no assignment, e.g., `$call_idr fp(10) then bb1`.

- validity requirements: in order to be a valid LIR program, the following requirements must be met:

    - the program is well-typed.

    - <u>the program contains a function `main : () -> int`, which is the entry point to the program.</u>

    - every struct type has at least one field.

    - <u>an extern function's name cannot be the same as an internally-defined function's name.</u>

    - for each function there must be exactly one basic block labeled `entry`, which is the entry point to the function.

    - for each function there must be exactly one basic block ending in a `$ret` (return) terminal.

    - all basic blocks in a function must be reachable from `entry` and all basic blocks must reach the basic block containing the `$ret` terminal.

    - every `$alloc` instruction has a unique bracketed identifier.

    - the `main` function cannot be called.

    - if a global variable has the same name as an internally-defined function then that variable is a function pointer to the associated function and has the appropriate type (but there cannot be a global variable named `main`).

# LIR grammar

- allows C++-style comments: `// comment up to end of line`

- blank lines and superfluous whitespace are ignored

- grammar:

```
# LIR program
program ::= (struct_def | global_def | extern_decl | function_def)+

# struct definition
struct_def ::= `struct` id `{` `\n` (id `:` type `\n`)+ `}` `\n`

# global variable definition
global_def ::= id `:` type `\n`

# extern function declaration
extern_decl ::= `extern` id `:` func_type `\n`

# function definition
function_def ::= `fn` id `(` parameters? `)` `->` return_type `{` `\n` body `}` `\n`

# function parameters
parameters ::= id `:` type (`,` id `:` type)*

# function return type or `_` indicating nothing is returned
return_type ::= type | `_`

# function body
body ::= decl? block+

# variable declarations
decl ::= `let` id `:` type (`,` id `:` type)* `\n`

# basic block
block ::= id `:` `\n` inst* term

# instruction
inst ::= id `=` `$addrof` id `\n`
       | id `=` `$alloc` operand `[` id `] `\n`
       | id `=` `$arith` aop operand operand `\n`
       | id `=` `$cmp` rop operand operand `\n`
       | id `=` `$copy` operand `\n`
       | id `=` `$gep` id operand `\n`
       | id `=` `$gfp` id id `\n`
       | id `=` `$load` id `\n`
       | `$store` id operand `\n`
       | (id `=`)? `$call_ext` id `(` (operand (`,` operand)*)? `)` `\n`

# arithmetic operation
aop ::= `add` | `sub` | `mul` | `div`

# relational comparison
rop ::= `eq` | `neq` | `lte` | `lt` | `gte` | `gt`

# terminal instruction
term ::= `$branch` operand id id `\n`
       | `$jump` id `\n`
       | `$ret` operand? `\n`
       | (id `=`)? `$call_dir` id `(` (operand (`,` operand)*)? `)` `then` id `\n`
       | (id `=`)? `$call_idr` id `(` (operand (`,` operand)*)? `)` `then` id `\n`

# instruction operand
operand ::= id | num

# integer constant
num ::= `-`? [0-9]+

# identifiers
id ::= ((`_`+ [a-zA-Z0-9]) | [a-zA-Z]) (`_` | `.` | [a-zA-Z0-9])*

# types
type ::= `int` | id | func_type | ptr_type

# a function type
func_type ::= `(` (type (`,` type)*) `)` `->` return_type

# a pointer type
ptr_type ::= `&` type
```

# LIR PEG parser combinator description

- this is the PEG description of the LIR grammar in case you find it useful (e.g., you plan to use a PEG library for parsing). the format is specific to the PEG library I used myself, so you'll probably need to tweak it if you want to use it. you don't need to use this at all, it should be pretty easy to manually write a parser instead.

```
WHITESPACE = _{ " " | "\t" }
COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* ~ &NEWLINE }

program = { SOI ~ NEWLINE* ~ ((struct_def ~ NEWLINE*) | (global_def ~ NEWLINE*) | (extern_decl ~NEWLINE*) | (function_def ~ NEWLINE*))+ ~ EOI }

struct_def = { struct_hdr ~ "{" ~ NEWLINE ~ field_def+ ~ "}" ~ NEWLINE }
struct_hdr = { "struct" ~ ident}
field_def = { ident ~ ":" ~ type_id ~ NEWLINE }

global_def = { ident ~ ":" ~ type_id ~ NEWLINE }

extern_decl = { "extern" ~ ident ~ ":" ~ func_typ ~ NEWLINE }

function_def = { function_hdr ~ "(" ~ parameters? ~ ")" ~ "->" ~ ret_ty ~ "{" ~ NEWLINE ~ body_def ~ "}" ~ NEWLINE }
function_hdr = { "fn" ~ ident }

parameters = _{ parameter ~ ("," ~ parameter)* }
parameter = { ident ~ ":" ~ type_id }

body_def = { NEWLINE* ~ decl? ~ NEWLINE* ~ (basic_block ~ NEWLINE*)+ }
decl = { "let" ~ local ~ ("," ~ local)* }
local = { ident ~ ":" ~ type_id }
basic_block = { ident ~ ":" ~ NEWLINE* ~ inst* ~ terminal }

inst = _{ (addrof | alloc | arith | callext | cmp | copy | gep | gfp | load | phi | store) ~ NEWLINE }
addrof = { ident ~ "=" ~ "$addrof" ~ ident }
alloc = { ident ~ "=" ~ "$alloc" ~ operand ~ "[" ~ ident ~ "]" }
arith = { ident ~ "=" ~ "$arith" ~ aop ~ operand ~ operand }
callext = { (ident ~ "=")? ~ "$call_ext" ~ ident ~ "(" ~ (operand ~ ("," ~ operand)*)? ~ ")" }
cmp = { ident ~ "=" ~ "$cmp" ~ rop ~ operand ~ operand }
copy = { ident ~ "=" ~ "$copy" ~ operand }
gep = { ident ~ "=" ~ "$gep" ~ ident ~ operand }
gfp = { ident ~ "=" ~ "$gfp" ~ ident ~ ident }
load = { ident ~ "=" ~ "$load" ~ ident }
phi = { ident ~ "=" ~ "$phi" ~ "(" ~ operand ~ ("," ~ operand)* ~")" }
ret = { "$ret" ~ operand? }
store = { "$store" ~ ident ~ operand }

terminal = { (branch | calldir | callidr | jump | ret) ~ NEWLINE }
branch = { "$branch" ~ operand ~ ident ~ ident }
calldir = { (ident ~ "=")? ~ "$call_dir" ~ ident ~ "(" ~ (operand ~ ("," ~ operand)*)? ~ ")" ~ "then" ~ ident }
callidr = { (ident ~ "=")? ~ "$call_idr" ~ ident ~ "(" ~ (operand ~ ("," ~ operand)*)? ~ ")" ~ "then" ~ ident }
jump = { "$jump" ~ ident }

aop = { "add" | "sub" | "mul" | "div" }
rop = { "eq" | "neq" | "lte" | "lt" | "gte" | "gt" }

type_id = { "int" | ident | func_typ | ptr_typ }
func_typ = { "(" ~ (type_id ~("," ~ type_id)*)? ~ ")" ~ "->" ~ ret_ty }
ret_ty = { "_" | type_id }
ptr_typ = { "&" ~ type_id }

ident = @{ ((("@" | "_")+ ~ ASCII_ALPHANUMERIC) | ASCII_ALPHA) ~ ("_" | "." | ASCII_ALPHANUMERIC)* }
num = @{ "-"? ~ ASCII_DIGIT+ }
operand = { ident | num }
```

# LIR example program

- the program is nonsense, it's just intended to show you what a program looks like.

```
struct bar {
    f1: int
    f2: &int
}

// global pointer to function foo.
foo: &(&int) -> int

// externally-defined function func.
extern func: (int) -> int

fn main() -> int {
    entry:
        $call_dir baz(42) then exit

    exit:
        $ret 0
}

fn baz(p:int) -> _ {
    let a:int, b:&int, c:&bar, d:&&int, fp:&(&int) -> int

    entry:
        a = $copy p
        b = $addrof a
        $store b 42
        c = $alloc 42 [_1]
        c = $gep c 12
        d = $gfp c f2
        a = $cmp lte a 3
        $branch a bb2 bb3

    bb2:
        b = $load d
        a = $call_dir foo(b) then bb4

    bb3:
        fp = $copy foo
        a = $call_idr fp(b) then bb4

    bb4:
        p = $arith add p 1
        a = $arith mul a p
        a = $call_ext bar(a)
        $jump exit

    exit:
        $ret
}

fn foo(p:&int) -> int {
    let r:int

    entry:
        r = $load p
        $ret r
}
```