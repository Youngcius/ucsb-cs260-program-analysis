# ASSIGNMENT 2

__RELEASED:__ Tuesday, Jan 30
__DUE:__ Tuesday, Feb 13
__LAST LATE DAY:__ Sunday, Feb 18

## PART 1: Reaching Definitions

Implement the reaching definitions analysis as described in lecture.

As a reminder: the abstract domain is `𝒫(PP)` (the powerset of the set of program points), where ⊥ = `{}` and ⊤ = `PP` and ⊔ (i.e., join) is set union. A program point is represented by `<basic block label>.{<index> | term}`, i.e., `bb1.2` is the instruction in basic block `bb1` with index 2 (starting from 0) and `bb1.term` is the terminal instruction of `bb1`.

Remember that we still have the set `addr_taken` that acts similarly to the integer-based analyses from assignment 1, but now it contains _all_ address-taken locals, _all_ globals, and also the fake variables representing types allocated on the heap. Unlike assignment 1, there _can_ be globals in the test programs. Globals can potentially have the same names as locals, so be sure to distinguish between them during your analysis (e.g., you could represent local variables with the string `<function name>.<id>` while globals would just be named `<id>`).

The output of your analysis should be a map from program points (containing uses) to sets of program points (containing the reaching defs of those uses):

```
<program point> -> { <program point>, ... }
...
```

Where the program points are listed in alphabetical order.

## PART 2: Control Dependence

Implement the control dominance analysis and compute the dominance frontier for each basic block as described in lecture.

As a reminder, for the dominance analysis the abstract domain is `𝒫(BasicBlock)` (the powerset of the set of basic blocks), where ⊥ = `BasicBlock` and ⊤ = `{}` and ⊔ (i.e., join) is set intersection.

The output of your analysis should be a map from basic blocks to sets of basic blocks (i.e., to the dominance frontier of each basic block):

```
<bb> -> { <bb>, ... }
...
```

Where the basic blocks are listed in alphabetical order.

## REFERENCE SOLUTIONS

I have placed executables of my own solutions to these analyses on CSIL in `~benh/260/{rdef, control}`. You can use these reference solutions to test your analyses before submitting. If you have any questions about the output format, you can answer them using these reference solutions as well; these are the solutions that Gradescope will use to test your submissions. My solutions only take two arguments (as opposed to the three that your solutions should take, described below): the file containing the LIR program and the name of the function to analyze.

## SUBMITTING TO GRADESCOPE

Your submission must meet the following requirements:

- There must be a `build-analyses.sh` bash script that does whatever is needed to build or setup both analyses (e.g., compile the code if necessary). If nothing is necessary the script must still exist, it can just to nothing.

- There must be `run-rdef.sh` and `run-control.sh` bash scripts that each take three command-line arguments: the first is a file containing the LIR program to analyze, the second is a file containing the same program in JSON format, and the third is the name of the function to analyze. You can use whichever program format you wish and ignore the other. Each script must print the result of the respective analysis to standard out.

- If your solution contains sub-directories then the entire submission must be given as a zip file (this is required by Gradescope for submissions that contain a directory structure). The scripts should all be at the root directory of the solution.

- The submitted files are not allowed to contain any binaries, and your solutions are not allowed to use the network. Your build scripts _are_ allowed to use the network if they need to install anything, but be wary of how much time they take (build time is part of the Gradescope timeout).

If you want to submit one analysis before you've implemented the other that's fine, but you still need all the scripts I mentioned (otherwise the grader will barf). The script for the missing analysis can just do nothing.

## GRADING

Here's how the grading will be broken down so that you can focus your work accordingly. There are two parts to the assignment (reaching defs and control dependence); reaching defs is worth 70 points and control dependence is worth 30 points.

The point breakdown for reaching definitions is (let P = pointers, S = structs, G = globals, F = function calls):

- Programs without PSGF: 25

- Programs without PSG, with F: 5

- Programs without PS, with GF: 10

- Programs without GF, with PS: 10

- Programs without F, with PSG: 10

- Programs with PSGF: 10

The point breakdown for control dependence is:

- Any valid program: 30

Each of these categories will have a test suite of LIR programs that will be used to test your submission on that category for the given analysis. You must get all tests in a given test suite correct in order to receive points for the corresponding category. You are encouraged to focus on one category at a time and get it fully correct before moving on to the next. Remember that you can also create your own test programs and use my solution on CSIL to see what my solution for that program would be.
