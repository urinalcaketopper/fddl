# fddl Programming Language

**fddl** is a small, experimental programming language, built in Rust, designed to explore language implementation concepts. It's a blend of ideas from various languages but remains unique, with its own syntax and quirks.

This project is both a learning experience and a passion project, where the goal is to build a working language from scratch, while embracing some of the core features I enjoy from other languages.

## Why fddl?

For years, I’ve tried to learn various programming languages, and while I could master the basics, the real-world projects often eluded me. Rust, however, clicked for me, and fddl was born out of this journey. fddl is my attempt to combine the aspects I appreciate from many languages into something uniquely my own.

---

## Current Features

- **Custom Syntax**: fddl introduces unique operators and keywords to make programming more intuitive and fun.
- **Lexer**: A working lexer that tokenizes fddl scripts into understandable pieces of the language.
- **Tilde Operator**: Includes a custom `~` and `~=` operator for creative syntax possibilities.
  
---

## Getting Started

To start experimenting with fddl, you can run it in two ways:

### Run the REPL (Interactive Mode)
```sh
cargo run
```

### Parse a fddl Script
```sh
cargo run path/to/script.fddl
```

## Running the Project

Make sure your project compiles and the tests pass:

```bash
cargo build
cargo test
```

---

## Examples

Here are a couple of examples showcasing the current capabilities of fddl:

### Hello World

```fddl
func main() {
    print("hello, world in fddl");
}
```

### Simple Math Module and Variable Declaration

```fddl
# This is a sample module

module math {
    # Computes the square of a number
    func square(x) => x ^ 2;
}

define $number := 5;
print(`The square of $number is ${math.square($number)}`);
```

(Note: This feature is under development, and string interpolation is planned for a future update.)

---

## Roadmap and Next Steps

fddl is very much a work in progress, with lots of planned improvements and additions. Here's a breakdown of what’s done and what’s coming up:

- **Lexer**: 
  - [x] Built and tested for basic syntax and operators.
  - [x] Supports single-line and documentation comments.
  - [ ] Add support for more complex syntax and features.

- **Parser**: 
  - [ ] Working on building out functions to parse simple functionality in the language (if, while, for), and to read their expressions and values
  - [ ] Implement parsing for function calls, expressions, checks, literally everything.

- **Compiler**: 
  - [ ] Currently a placeholder. Implement the compiler to compile parsed code.

- **Comments**:
  - [x] Added support for single-line and documentation comments.
  - [ ] Implement multi-line comments.
  - [ ] Implement document building comments.

- **Error Handling**: 
  - [ ] Replace `stderr` with a more robust error handling mechanism.

- **Testing**: 
  - [x] Added initial `lexer` tests. 
  - [ ] Expand tests to cover more syntax and edge cases.

---

## License

This project is licensed under the MIT License.
