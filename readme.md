# fddl Programming Language

**fddl** is a small, experimental programming language, built in Rust, designed to explore language implementation concepts. It's a blend of ideas from various languages but remains unique, with its own syntax and quirks.

This project is both a learning experience and a passion project, where the goal is to build a working language from scratch, while embracing some of the core features I enjoy from other languages.

## Why fddl?

For years, I’ve tried to learn various programming languages, and while I could master the basics, the real-world projects often eluded me. Rust, however, clicked for me, and fddl was born out of this journey. fddl is my attempt to combine the aspects I appreciate from many languages into something uniquely my own.

## No, but why "fddl", as in the name?

Oh, I see...

Well, Ferris, the mascot of Rust, is a crab. There's a type of crab that exists called a Fiddler Crab. So, yeah, that.

---

## Current Features

-   **Custom Syntax**: `fddl` introduces unique operators and keywords.
-   **Lexer**:
    -   Tokenizes `fddl` scripts, handling various operators, literals (numbers, strings, booleans, nil), and keywords.
    -   Supports single-line comments (`//`, `#`) and multi-line block comments (`/* ... */`).
    -   Keywords include `let`, `func`, `return`, `if`, `else`, `while`, `for`, `print`, `true`, `false`, `nil`, `and`, `or`, `some`, `not`, and more.
-   **Parser**:
    -   Builds an Abstract Syntax Tree (AST) from the token stream.
    -   **Comprehensive Expression Parsing**:
        -   Literals: Numbers, strings, booleans, `nil`.
        -   Unary Operations: `-` (negation), `~` (almost), `some`, `not`.
        -   Binary Operations: Handles arithmetic (`+`, `-`, `*`, `/`, `%`), comparisons (`<`, `<=`, `>`, `>=`), equality (`==`, `!=`), and logical (`and`, `or`) operators with correct precedence and associativity.
        -   Grouping: Parenthesized expressions `(...)`.
        -   Function Calls: Parses `function_name(arg1, arg2, ...)` with complex expressions as arguments.
    -   **Statement Parsing**:
        -   `print` statements.
        -   `let` variable declaration statements.
        -   Assignment statements (`identifier = expression;`).
        -   Block statements (`{ ... }`) for grouping multiple statements.
        -   `if-else if-else` control flow statements with block bodies.
        -   `while` loop statements with block bodies.
        -   `for` loop statements (C-style: `for (initializer; condition; increment) { body }`, including `let` initializers) with block bodies.
        -   `func` function declaration statements (name, parameters, block body).
        -   `return` statements (with optional expression).
        -   Expression statements.
    -   Skips comment tokens during parsing.
-   **Basic Interpreter (Ongoing)**:
    -   Tree-walking interpreter for executing ASTs.
    -   Evaluates literal expressions (numbers, strings, booleans, nil).
    -   Evaluates unary minus (`-`) expressions.
    -   Evaluates binary arithmetic expressions (`+`, `-`, `*`, `/`, `%`) including division-by-zero checks.
    -   Evaluates grouping expressions `()`.
    -   Executes `PrintStatement` and `ExpressionStatement`.
-   **Tilde Operator**: Includes a custom `~` (unary "Almost") and `~=` (binary "AlmostEqual" - lexed, parser TBD) operator.

---


## Getting Started

To get started with `fddl`, you'll first need to have the Rust programming language and its package manager, Cargo, installed on your system. If you don't have them yet, you can find installation instructions on the official Rust website: [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are set up:

1.  **Clone the Repository** (if you haven't already):
    ```sh
    # Replace <repository_url> with the actual URL of your fddl repository
    git clone <repository_url>
    cd fddl
    ```

2.  **Build the Project**:
    You can build the project using Cargo:
    ```sh
    cargo build
    ```
    This will compile `fddl` and place the executable in the `target/debug/` directory.

After building, you can run `fddl` using `cargo run` (which compiles and then runs).

Mind you, there isn't much there currently. The REPL only returns minimal information currently.

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

(Note: This feature is under development, and string interpolation is planned for the future.)

---

## Roadmap and Next Steps

`fddl` is very much a work in progress.

-   **Lexer**:
    -   [x] Core functionality built and tested.
    -   [x] Supports single-line (`//`, `#`) and multi-line block comments (`/* ... */`).
    -   [ ] Consider advanced features like escape sequences in strings more thoroughly.
-   **Parser**:
    -   [x] Comprehensive expression parsing (primary, unary (`-`, `~`, `some`, `not`), binary with precedence (arithmetic, comparison, equality, logical), grouping, function calls).
    -   [x] Core statement parsing (`print`, `let`, assignment, `if/else`, `while`, `for` (with `let` initializers), blocks (`{...}`), `func` declaration, `return`).
    -   [ ] L & R Values: Formalize for assignment and other contexts (more a semantic/compiler concern).
    -   [ ] Potentially parse types for type checking later if `fddl` becomes statically typed.
-   **Interpreter (Current Focus)**:
    -   [x] Basic tree-walking framework for AST evaluation.
    -   [x] Evaluation of literal expressions (numbers, strings, booleans, `nil`).
    -   [x] Evaluation of unary minus (`-`) expressions.
    -   [x] Evaluation of binary arithmetic expressions (`+`, `-`, `*`, `/`, `%`) including division-by-zero checks.
    -   [x] Evaluation of grouping expressions `()`.
    -   [x] Execution of `PrintStatement` and `ExpressionStatement`.
    -   [ ] Implement evaluation for remaining unary operators (`not`, `some`, `~`).
    -   [ ] Implement evaluation for binary comparison (`<`, `<=`, `>`, `>=`), equality (`==`, `!=`), and logical (`and`, `or`) operators.
    -   [ ] **Environment for Variables**: Implement variable declaration (`let`), assignment (`=`), and lookup (`identifier`).
    -   [ ] **Control Flow Execution**: `if/else`, `while`, `for`.
    -   [ ] **Function Execution**: Handling function calls, parameter passing, environments/scopes, and `return` statements.
-   **Compiler**:
    -   [ ] Currently a placeholder. Future goal: Implement a compiler (e.g., to bytecode or another target).
-   **Error Handling**:
    -   [ ] Improve error reporting with more precise location information (line/column) consistently across lexer, parser, and interpreter.
    -   [ ] Consider "errors as values" as a language feature (currently a back-burner idea).
-   **Testing**:
    -   [x] Added initial `lexer` tests.
    -   [ ] Expand tests to cover parser AST output more systematically.
    -   [ ] Add tests for interpreter behavior as features are implemented.
-   **REPL/Tooling**:
    -   [ ] Enhance REPL for multi-line input.
    -   [ ] Ensure robust file input processing in `main.rs`.

---

## License

This project is licensed under the GPL License.
