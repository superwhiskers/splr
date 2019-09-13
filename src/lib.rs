/*!
# a SAT Solver for Propositional Logic in Rust

Splr is a pure Rustic SAT solver, based on [Glucose 4.1](https://www.labri.fr/perso/lsimon/glucose/).
It adopts various research results on SAT solvers:

- CDCL, watch literals, VSIDS and so on from [Minisat](http://minisat.se) and the ancestors
- Glucose-like dynamic blocking/forcing restarts based on [EMAs](https://arxiv.org/abs/1506.08905)
- heuristics adaptation
- pre/in-process simplification based on clause subsumption and variable elimination

*Many thanks to SAT researchers.*

## Usage

Splr is a standalone program, taking a CNF file. The result will be saved to a file.

```plain
$ splr tests/sample.cnf
sample.cnf                                         250,1065 |time:     0.68
 #conflict:      33526, #decision:        38700, #propagate:        1521424
  Assignment|#rem:      235, #fix:        1, #elm:       14, prg%:   6.0000
 Clause Kind|Remv:    11807, LBD2:       75, Binc:        1, Perm:     1079
     Restart|#BLK:      403, #RST:        0, eASG:   1.4764, eLBD:   1.0034
    Strategy|mode: in the initial search phase to determine a main strategy
SATISFIABLE: tests/sample.cnf. The result was saved to ./.ans_sample.cnf.

$ cat .ans_sample.cnf
c An assignment set generated by splr-0.2.0-alpha.0 for tests/sample.cnf
c
c sample.cnf                                 , #var:      250, #cls:     1065
c  #conflict:      33526, #decision:        38700, #propagate:        1521424
c   Assignment|#rem:      235, #fix:        1, #elm:       14, prg%:   6.0000
c  Clause Kind|Remv:    11807, LBD2:       75, Binc:        1, Perm:     1079
c      Restart|#BLK:      403, #RST:        0, eASG:   1.4764, eLBD:   1.0034
c    Conflicts|aLBD:     8.20, bjmp:      NaN, cnfl:      NaN |blkR:   1.4000
c    Clause DB|#rdc:        5, #sce:        2, #exe:        0 |frcK:   0.8200
c     Strategy|mode:        initial, time:     0.66
c
s SATISFIABLE
1 2 3 4 -5 6 7 -8 -9 10 -11 -12 -13 -14 15 16 -17 18 -19 -20 -21 -22 23 ... 0

$ dmcr tests/sample.cnf
Valid assignment set for tests/sample.cnf found in .ans_sample.cnf.
```

The answer file uses the following format.

- It contains a single line starting with `s` and followed by `SATISFIABLE` or `UNSATISFIABLE`.
- It ends a line of assignments separated by a space and `0` as EOL, if the problem is satisfiable.
  Otherwise it contains only `0`.
- Lines starting with `c` are comments, used for dumping statistics

### Mnemonics in progress message

| mnemonic  | meaning |
| --------- |------- |
| `v`  | the number of variables used in the given CNF file |
| `c`  | the number of clauses used in the given CNF file |
| `time`  | elapsed CPU time in seconds (or wall-clock time if CPU time is not available) |
| `#conflict` | the number of conflicts |
| `#decision` | the number of decisions |
| `#propagate` | the number of propagates (its unit is literal) |
| `#rem` | the number of remaining variables |
| `#fix` | the number of solved variables (which has been assigned a value at decision level zero) |
| `#elm` | the number of eliminated variables |
| `prg%` | the percentage of `remaining variables / total variables` |
| `Remv` | the number of learnt clauses which are not biclauses |
| `LBD2` | the number of learnt clauses which LBDs are 2 |
| `Binc` | the number of binary learnt clauses |
| `Perm` | the number of given clauses and binary learnt clauses |
| `#BLK` | the number of blocking restart |
| `#RST` | the number of restart |
| `eASG` | a moving rate of the number of assigned variables |
| `eLBD` | a moving rate of earn clause's LBD |
| `aLBD` | the EMA, Exponential Moving Average, of learn clauses' LBDs |
| `cnfl` | the EMA of decision levels to which backjumps go |
| `bjmp` | the EMA of decision levels at which conflicts occur |
| `rpc%` | a percentage of restart per conflict |
| `#rdc` | the number of `reduce` invocations |
| `#sce` | the number of satisfied clause eliminations done by `simplify` |
| `blkR` | the coefficient for blocking restart, called 'R' in Glucose |
| `frcK` | the coefficient for forcing restart, called 'K' in Glucose |
| `mode` | Selected strategy's id |
| `time` | the elapsed CPU time in seconds |

## Command line options

Please check help message.

```plain
$ splr --help
splr 0.1.4
Shuji Narazaki <shujinarazaki@protonmail.com>
A pure rustic CDCL SAT solver based on Glucose

USAGE:
    splr [FLAGS] [OPTIONS] <cnf-filename>

FLAGS:
    -h, --help                         Prints help information
    -c, --certify                      Writes a DRAT UNSAT certification file
    -l, --log                          Uses Glucose-like progress report
    -V, --version                      Prints version information
    -R, --without-adaptive-restart     Disables dynamic restart adaptation
    -S, --without-adaptive-strategy    Disables dynamic strategy adaptation
    -D, --without-deep-search          Disables deep search mode
    -E, --without-elim                 Disables exhaustive simplification

OPTIONS:
        --cl <clause-limit>           soft limit of #clauses (24MC~4GB) [default: 0]
        --stat <dump-interval>        interval for dumpping stat data [default: 0]
        --eg <elim-grow-limit>        grow limit of #clauses by v-elim [default: 4]
        --el <elim-lit-limit>         #literals in a clause by v-elim [default: 64]
    -o, --dir <output-dirname>        output directory [default: .]
    -p, --proof <proof-filename>      filename for DRAT cert. [default: proof.out]
        --ra <restart-asg-len>        length for assignment average [default: 3500]
        --rb <restart-blocking>       blocking restart threshold [default: 1.40]
        --rl <restart-lbd-len>        length for LBD average [default: 50]
        --rs <restart-step>           #conflicts between restarts [default: 50]
        --rt <restart-threshold>      forcing restart threshold [default: 0.70]
    -r, --result <result-filename>    result filename/stdout [default: ]
        --to <timeout>                CPU time limit in sec. [default: 0]

ARGS:
    <cnf-filename>    a DIMACS format CNF file
```

## Correctness

While Splr comes with **ABSOLUTELY NO WARRANTY**, Splr version 0.1.0 (splr-0.1.0) was verified with the following problems:

* The first 100 problems from
  [SATLIB](https://www.cs.ubc.ca/~hoos/SATLIB/benchm.html),
  [250 variables uniform random satisfiable 3-SAT](https://www.cs.ubc.ca/~hoos/SATLIB/Benchmarks/SAT/RND3SAT/uf250-1065.tar.gz)
  : all the solutions are correct.
* The first 100 problems from
  [SATLIB](https://www.cs.ubc.ca/~hoos/SATLIB/benchm.html),
  [250 variables uniform random unsatisfiable 3-SAT](https://www.cs.ubc.ca/~hoos/SATLIB/Benchmarks/SAT/RND3SAT/uuf250-1065.tar.gz)
  : all the solutions are correct and verified with [drat-trim](http://www.cs.utexas.edu/~marijn/drat-trim/).
* [SAT Competition 2017](https://baldur.iti.kit.edu/sat-competition-2017/index.php?cat=tracks),
  [Main track](https://baldur.iti.kit.edu/sat-competition-2017/benchmarks/Main.zip)
  : with a 2000 sec timeout, splr-0.1.0 solved:
  * 72 satisfiable problems: all the solutions are correct.
  * 51 unsatisfiable problems: [Lingeling](http://fmv.jku.at/lingeling/) or Glucose completely returns the same result. And,
     * 37 certificates generated by splr-0.1.1 were verified with drat-trim.
     * The remaining 14 certificates weren't able to be verified due to [timeout](https://gitlab.com/satisfiability01/splr/issues/74#note_142021555) by drat-trim.
*/
// /// Subsumption-based clause/var elimination
/// Clause structure
pub mod clause;
/// Parameters used for Solver initialization
pub mod config;
/// Pre/In-processor for clause subsumption and variable elimination
pub mod eliminator;
/// Assignment management
pub mod propagator;
/// Solver restart implementation
pub mod restart;
/// The main structure
pub mod solver;
/// Collection of various data and parameters for SAT solving process
pub mod state;
/// Interfaces between submodules
pub mod traits;
/// Plumping layer
pub mod types;
/// validates a given assignment for a problem.
pub mod validator;
/// Var structure
pub mod var;

#[macro_use]
extern crate bitflags;
