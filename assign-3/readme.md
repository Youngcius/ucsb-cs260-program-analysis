# ASSIGNMENT 3

__RELEASED:__ Tuesday, Feb 13
__DUE:__ Tuesday, Feb 27
__LAST LATE DAY:__ Sunday, Mar 3

This assignment is about implementing an Andersen-style pointer analysis. I've broken the assignment into two independent pieces so that you can get full marks for one even if you have problems with the other. If you put them together then you get the full Andersen-style pointer analysis.

## PART 1: Andersen-Style Pointer Analysis Constraint Generator

Implement constraint generation for a field-insensitive Andersen-style pointer analysis. The generator will take a LIR program as input and output a list of constraints in the format described below.

Each line of the output will be a constraint of the form `<exp> <= <exp>`, where `<exp>` can be:

- `<var>` for set variables, in the form of:
    - `<variable name>` (for globals and `$alloc` identifiers)
    - `<function name>.<variable name>` (for function parameters and locals)

- `ref(<var>,<var>)` for `ref` calls (we'll use the program variable name as both the constant and set variable arguments)

- `proj(ref,1,<var>)` for projections (we'll only ever be projecting on `ref` constructor calls, and always on position 1)

- `lam_[<type>](<arg>,...)` for `lam` calls (where `<type>` is the type of the function)

The constraints should be listed in alphabetical order. For example, the input:

```
foo: &(&int,int,&int)->&int

fn foo(p1:&int, p2:int, p3:&int) -> &int {
let r:&int
entry:
    r = $copy p3
    $ret r
}

fn main() -> int {
let a:&int, b:&int, c:&int, d:&&int, e:&int, f:&(&int,int,&int)->&int, g:int
entry:
    a = $alloc 1 [_alloc1]
    b = $alloc 1 [_alloc2]
    f = $copy foo
    c = $call_idr f(a, 42, b) then exit

exit:
    d = $alloc 1 [_alloc3]
    $store d a
    e = $load d
    g = $load e
    $ret g
}
```

Will output the following:

```
foo <= main.f
foo.p3 <= foo.r
lam_[(&int,int,&int)->&int](foo,foo.r,foo.p1,foo.p3) <= foo
main.a <= proj(ref,1,main.d)
main.f <= lam_[(&int,int,&int)->&int](_DUMMY,main.c,main.a,main.b)
proj(ref,1,main.d) <= main.e
ref(_alloc1,_alloc1) <= main.a
ref(_alloc2,_alloc2) <= main.b
ref(_alloc3,_alloc3) <= main.d
```

## PART 2: Andersen-Style Pointer Analysis Constraint Solver

Implement a constraint solver for a field-insensitive Andersen-style pointer analysis. The solver will take a file containing a list of constraints in the same format as the output of Part 1. It should output a solution mapping program variables to their points-to sets (where all variable are listed in alphabetical order).

Note: all lam constructor calls have the first argument as covariant; whether the second argument is covariant or contravariant depends on whether the function returns a pointer or not. You can easily determine this information based on the type information given as part of the lam name (`lam_[<type>]`): if the type contains `->&` then the function returns a pointer, in which case the second argument is covariant, otherwise it is contravariant. All other arguments are always contravariant.

For example, the input:

```
foo <= main.f
foo.p3 <= foo.r
lam_[(&int,int,&int)->&int](foo,foo.r,foo.p1,foo.p3) <= foo
main.a <= proj(ref,1,main.d)
main.f <= lam_[(&int,int,&int)->&int](_DUMMY,main.c,main.a,main.b)
proj(ref,1,main.d) <= main.e
ref(_alloc1,_alloc1) <= main.a
ref(_alloc2,_alloc2) <= main.b
ref(_alloc3,_alloc3) <= main.d
```

Will output the following:

```
_alloc3 -> {_alloc1}
foo -> {foo}
foo.p1 -> {_alloc1}
foo.p3 -> {_alloc2}
foo.r -> {_alloc2}
main.a -> {_alloc1}
main.b -> {_alloc2}
main.c -> {_alloc2}
main.d -> {_alloc3}
main.e -> {_alloc1}
main.f -> {foo}
```

## REFERENCE SOLUTIONS

I have placed executables of my own solutions to these analyses on CSIL in `~benh/260/{constraint-generator, constraint-solver}`. You can use these reference solutions to test your analyses before submitting. If you have any questions about the output formats, you can answer them using these reference solutions as well; these are the solutions that Gradescope will use to test your submissions.

IMPORTANT NOTE: My `constraint-solver` solution assumes for its input that `<exp>` does not contain any whitespaces (which is true for the examples above and for what is output by my `constraint-generator` solution). Your solutions don't need to make this assumption or enforce it (the autograder will work regardless), but:

    - You can rely on this assumption for the inputs to your solution when tested by the autograder.
    
    - If you want to test something using my `constraint-solver` solution it will only work if this assumption is true.

## SUBMITTING TO GRADESCOPE

Your submission must meet the following requirements:

- There must be a `build-analyses.sh` bash script that does whatever is needed to build or setup both analyses (e.g., compile the code if necessary). If nothing is necessary the script must still exist, it can just to nothing.

- There must be `run-solver.sh` and `run-generator.sh` bash scripts; `run-solver.sh` should take one argument: a file containing a list of constraints; `run-generator.sh` should take two arguments: a file containing the LIR program to generate constraints for and a file containing the JSON representation of that program. Each script must print the result of the respective analysis to standard out.

- If your solution contains sub-directories then the entire submission must be given as a zip file (this is required by Gradescope for submissions that contain a directory structure). The scripts should all be at the root directory of the solution.

- The submitted files are not allowed to contain any binaries, and your solutions are not allowed to use the network. Your build scripts _are_ allowed to use the network if they need to install anything, but be wary of how much time they take (build time is part of the Gradescope timeout).

If you want to submit one analysis before you've implemented the other that's fine, but you still need all the scripts I mentioned (otherwise the grader will half). The script for the missing analysis can just do nothing.

## GRADING

Here's how the grading will be broken down so that you can focus your work accordingly:

PART 1:

- Programs with no calls: 35

- Programs with direct calls: 5

- Programs with direct and indirect calls: 10

PART 2:

- No non-ref constructor calls and no projections: 5

- No non-ref constructor calls: 35

- With lam constructor calls: 10

Each of these categories will have a test suite that will be used to test your submission on that category for the given analysis. You must get all tests in a given test suite correct in order to receive points for the corresponding category. You are encouraged to focus on one category at a time and get it fully correct before moving on to the next. Remember that you can also create your own test programs and use my solution on CSIL to see what my solution for that program would be.
