# ASSIGNMENT 1

__RELEASED:__ Tuesday, Jan 16
__DUE:__ Tuesday, Jan 30
__LAST LATE DAY:__ Sunday, Feb 4

## PART 1: Integer Constant Analysis

Implement the <u>intraprocedural</u> integer constant analysis using the MFP worklist algorithm, as described in lecture.

As a reminder: The abstract domain is `⊥ ⊑ {..., -2, -1, 0, 1, 2, ...} ⊑ ⊤`, where `⊥` means "no integer value", `⊤` means "any integer value", and for `n` ∈ 𝐙, `n` means "exactly the value `n`". The join of `⊥` and any abstract value `X` is `X`; the join of any abstract value with itself is itself; otherwise the join of any two abstract values is `⊤`.

You must devise the abstract semantics for the arithmetic operators (`add`, `sub`, `mul`, `div`) and comparison operators (`eq`, `neq`, `lt`, `lte`, `gt`, `gte`) using this abstract domain (i.e., fill in the entries of the table below for each operator, where `c1` and `c2` represent integer constants).
```
<operator> | ⊥ | c2 | ⊤
-----------+---+----+---
         ⊥ |   |    |
        c1 |   |    |
         ⊤ |   |    |
```

**Abstract semantics**

- arithmetic operators (`⊔`: `add`/`sub`/`mul`/`div`)

  | ⊔ | $\bot$ | $c_2$          | $\top$ |
  | ----------------------- | ------ | -------------- | ------ |
  | $\bot$                  | $\bot$ | $\bot$         | $\bot$ |
  | $c_1$                   | $\bot$ | $c_1 ⊔ c_2$ | $\top$ |
  | $\top$                  | $\bot$ | $\top$         | $\top$ |

    For the arithmetic operators (`add`, `sub`, `mul`, `div`), the abstract semantics using the join operation `⊔` are as follows:

    - `add`: `X ⊔ Y`
    - `sub`: `X ⊔ Y`
    - `mul`: `X ⊔ Y`
    - `div`: `X ⊔ Y`, where `Y ≠ 0`; $\bot$, where `Y = 0` 

- comparison operators  (`⊔`: `eq`/`neq`/`lt`/`lte`/`gt`/`gte`)

  | ⊔      | $\bot$ | $c_2$                | $\top$ |
  | ------ | ------ | -------------------- | ------ |
  | $\bot$ | $\bot$ | $\bot$               | $\bot$ |
  | $c_1$  | $\bot$ | $c_1 ⊔ c_2$ `as i32` | $\top$ |
  | $\top$ | $\bot$ | $\top$               | $\top$ |


## PART 2: Integer Interval Analysis (aka Range Analysis)

Implement the <u>intraprocedural</u> integer interval analysis using the MFP worklist algorithm and a widening operator (applied only at loop headers), as described in lecture. 

As a reminder: The abstract domain elements are `⊥` (the empty interval) and intervals `[a, b]` where `a` ∈ 𝐙 ∪ {-∞} and `b` ∈ 𝐙 ∪ {∞} and `a` <= `b`. In this domain, `⊤` = `[-∞, ∞]`. The join of `⊥` with any abstract value `X` is `X`; the join of two intervals `I1` and `I2` is `[min(I1.low, I2.low), max(I1.high, I2.high)]`. The widening of `⊥` with any abstract value `X` is `X`; otherwise `widen(I1, I2)` = `I3` s.t.

- `I3.low` = `I1.low` if `I1.low` <= `I2.low`, otherwise `-∞`
- `I3.high` = `I1.high` if `I1.high` >= `I2.high`, otherwise `∞`

You must devise the abstract semantics for the arithmetic operators (`add`, `sub`, `mul`, `div`) and comparison operators (`eq`, `neq`, `lt`, `lte`, `gt`, `gte`) using this abstract domain. The comparison operators are straightforward (remember that comparison always returns either `0` [i.e., false] or `1` [i.e., true]). For the arithmetic operators the simplest method is to compute all possible values using the input interval bounds and then select the minimum and maximum as the new interval bounds. For example, given `[-2, 3] * [-1, 1]`: `-2 * -1 = 2`, `-2 * 1 = -2`, `3 * -1 = -3`, `3 * 1 = 3`, therefore the new interval is `[-3, 3]`. Division is a little trickier because division by zero is undefined; we handle it like so for `I1 ÷ I2`:

- If `I2` = `[0, 0]`: the answer is `⊥`.
- If `I2.low` is negative and `I2.high` is positive (i.e., the interval contains 0): treat this as `I1 ÷ [-1, 1]` using the min/max method given above.
- If `I2.low` is 0: treat this as `I1 ÷ [1, I2.high]` using the min/max method given above.
- If `I2.high` is 0: treat this as `I1 ÷ [I2.low, -1]` using the min/max method given above.
- Otherwise just use the min/max method directly.

**Abstract semantics**

- arithmetic operators (`⊔`: `add`/`sub`/`mul`/`div`)

  | ⊔      | $\bot$ | $I_2$       | $\top$ |
  | ------ | ------ | ----------- | ------ |
  | $\bot$ | $\bot$ | $\bot$      | $\bot$ |
  | $I_1$  | $\bot$ | $T_1 ⊔ T_2$ | $\top$ |
  | $\top$ | $\bot$ | $\top$      | $\top$ |

    For the arithmetic operators (`add`, `sub`, `mul`, `div`), the abstract semantics using the join operation `⊔` are as follows:

    - `add`: `X ⊔ Y`
    - `sub`: `X ⊔ Y`
    - `mul`: `X ⊔ Y`
    - `div`: `X ⊔ Y`, where `Y ≠ 0`; $\bot$, where `Y = 0` 

- comparison operators  (`⊔`: `eq`/`neq`/`lt`/`lte`/`gt`/`gte`)

  | ⊔      | $\bot$ | $I_2$       | $\top$ |
  | ------ | ------ | ----------- | ------ |
  | $\bot$ | $\bot$ | $\bot$      | $\bot$ |
  | $I_1$  | $\bot$ | $T_1 ⊔ T_2$ | $\top$ |
  | $\top$ | $\bot$ | $\top$      | $\top$ |

## ANALYSIS OUTPUT

The result of each analysis should be, for the analyzed function, a map from each basic block to the abstract values _at the end_ of that basic block for all variables that are not `⊥`. Your solution should print the analysis results to standard output in the following form:

```
<basic block label>:
<variable name 1> -> <abstract value>
<variable name 2> -> <abstract value>
...and so on

<basic block label>:
<variable name 1> -> <abstract value>
<variable name 2> -> <abstract value>
...and so on

...and so on
```

Where the blocks are printed in alphabetical order and, for each block, the variables are printed in alphabetical order. Whitespace doesn't matter (e.g., exact number of spaces, etc).

## REFERENCE SOLUTIONS

I have placed executables of my own solutions to these analyses on CSIL in `~benh/260/{constants, intervals}`. You can use these reference solutions to test your analyses before submitting. If you have any questions about the output format, you can answer them using these reference solutions as well; these are the solutions that Gradescope will use to test your submissions. My solutions only take two arguments (as opposed to the three that your solutions should take, described below): the file containing the LIR program and the name of the function to analyze.

Usage of `constants` and `intervals` :

``` shell
./constants ./tests/test.1.1.lir ...
```

## SUBMITTING TO GRADESCOPE

Your submission must meet the following requirements:

- There must be a `build-analyses.sh` bash script that does whatever is needed to build or setup both analyses (e.g., compile the code if necessary). If nothing is necessary the script must still exist, it can just to nothing.

- There must be `run-constants-analysis.sh` and `run-intervals-analysis.sh` bash scripts that each take three command-line arguments: the first is a file containing the LIR program to analyze, the second is a file containing the same program in JSON format, and the third is the name of the function to analyze. You can use whichever program format you wish and ignore the other. Each script must print the result of the respective analysis to standard out.

- If your solution contains sub-directories then the entire submission must be given as a zip file (this is required by Gradescope for submissions that contain a directory structure). The scripts should all be at the root directory of the solution.

- The submitted files are not allowed to contain any binaries, and your solutions are not allowed to use the network.

If you want to submit one analysis before you've implemented the other that's fine, but you still need all the scripts I mentioned (otherwise the grader will barf). The script for the missing analysis can just do nothing.

## GRADING

Here's how the grading will be broken down so that you can focus your work accordingly. There are two parts to the assignment (constants analysis and interval analysis), each worth 50 points. For each part, the point breakdown will be:

- Programs with no pointers or function calls: 25

- Programs with no pointers but with function calls: 5

- Programs with pointers but no function calls: 10

- Programs with pointers and function calls: 10

Each of these categories will have a test suite of LIR programs that will be used to test your submission on that category for the given analysis. You must get all tests in a given test suite correct in order to receive points for the corresponding category. You are encouraged to focus on one category at a time and get it fully correct before moving on to the next. Remember that you can also create your own test programs and use my solution on CSIL to see what my solution for that program would be.
