# RustScript
### By: Logan Spinali
A subset PostScript interpreter implemented in Rust

This interpreter supports stack-based execution, dictionary scoping, procedures, and both dynamic (default) and lexical (static) scoping modes.

## Installation
### Method 1: building from files
#### Prerequisites:
* install rustup and cargo (https://rust-lang.org/tools/install/)
#### Steps:
* open a terminal in the RustScript folder
* cmd: "cargo run --release" to run the interpreter
* cmd: "cargo test" to run the test suite
### Method 2: release version
* go to GitHub repo: https://github.com/VortexCoreCoding/RustScript.git
* download the latest release in the releases section
* start the executable (I swear its not a virus)

## Commands
* Stack Manipulation: exch, pop, copy, dup, clear, count
* Arithmetic: add, sub, mul, div, idiv, mod, abs, neg, ceiling, floor, round, sqrt
* Dictionary: dict, length, maxlength, begin, end, def
* Strings: length, get, getinterval, putinterval
* Boolean/Bitwise: eq, ne, ge, gt, le, lt, and, or, not, true, false
* Flow Control: if, ifelse, for, repeat, quit
* Output: print, =, ==
* Special/Custom: help, -s, -l, -d, -m

## Rust Experience
This was my first time using Rust, so I had to learn all the basic syntax for the language as I made this project. There were a few curveballs that made Rust different to work with than C/C++/C#. The first was Rust's strict ownership model; when a complex variable is stored by two vars, Rust will make the former one lose access to the variable data. This can be dealt with by using Clone(), but it was an interesting workflow for this project. The second major difference I encountered was mutability, which by default is disallowed for a variable, but can be allowed by using keyword mut when defining a variable. All commands required for the project were able to be implemented, at least for the frontend experience, though the backend for string modifications is a lot of cloning rather than direct modification.