# RustScript
### By: Logan Spinali
A subset PostScript interpreter implemented in Rust

This interpreter supports stack-based execution, dictionary scoping, procedures, and both dynamic (default) and lexical (static) scoping modes.

## GitHub Repo:
https://github.com/VortexCoreCoding/RustScript.git


## Installation
### Prerequisites:
* install rustup and cargo (https://rust-lang.org/tools/install/)
* open a terminal in the RustScript folder
* cmd: "cargo run" to run the interpreter
* cmd: "cargo test" to run the test suite

## Commands
* Stack Manipulation: exch, pop, copy, dup, clear, count
* Arithmetic: add, sub, mul, div, idiv, mod, abs, neg, ceiling, floor, round, sqrt
* Dictionary: dict, length, maxlength, begin, end, def
* Strings: length, get, getinterval, putinterval
* Boolean/Bitwise: eq, ne, ge, gt, le, lt, and, or, not, true, false
* Flow Control: if, ifelse, for, repeat, quit
* Output: print, =, ==
* Special/Custom: help, -s, -l, -d, -m