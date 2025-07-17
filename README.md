# Fork Language Interpreter

A simple interpreter for a custom scripting language (`.fork` files) implemented in Rust. Supports control flow constructs (like `for` loops), arithmetic expressions, assignments, and print statements.

## Features
- **For Loops:** C-style `for` loops, including variable initialization, condition, and increment.
- **Arithmetic Expressions:** Supports `+`, `-`, `*`, `/` in assignments and print statements.
- **Print Statements:** Print variables, string literals, and arithmetic expressions.
- **Variable Assignment:** Supports variables and re-assignment.
- **File Extension Enforcement:** Only files with `.fork` extension can be run.
- **Clean Modular Code:** Refactored for maintainability; control flow handled in dedicated modules.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- Git (to clone the repo)

### Clone the Repository
```sh
git clone https://github.com/Sahildhillon0/fork.git
cd fork
```

### Build
```sh
cargo build
```

### Usage

#### Run a `.fork` File
```sh
cargo run ./src/test.fork
```
- Only files with the `.fork` extension are accepted.
- Example output:
  ```
  x =  20
  y*7 =  0
  y*7 =  7
  ...
  ```

#### Example `.fork` Program
```fork
x = 20;
print("x = ", x);
for (int y = 0; y < x; y = y + 1) {
    print("y*7 = ", y*7);
}
```

#### REPL Mode
If you run with no arguments:
```sh
cargo run
```
You get an interactive prompt:
```
fork> x = 5;
fork> print("x = ", x);
x =  5
fork> 
```
Type `exit` or `quit` to leave the REPL.

## Project Structure
- `src/main.rs` - Entry point, file/repl handling, extension check
- `src/interpreter/` - Interpreter logic
  - `control/` - Control flow modules (for, loops, condition)
  - `statement.rs` - Assignment and print logic
  - `context.rs` - Variable storage
- `src/test.fork` - Example program

## Contributing
Pull requests are welcome!

## License
MIT
