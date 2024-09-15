# fddl Programming Language

fddl is a small programming language inspired by various languages, designed to help learn language implementation concepts in Rust.

I have, off and on throughout the last 15 or so years attempted to learn a programming language of some sort. I could always get through the basics, but would get stuck with any real world projects. And I wouldn't know who to turn to even if I knew where to start.

So I started learning Rust and really like it. So I've been following some tutorials and the Crafting Interpretors site as guides for this very problematic programming language. 

I like aspects of so many programming languages, but I don't really like any of them, so I always found it hard to pick one and stick with it. But I had the same problem playing World of Warcraft, too. 

So I, like many of you, decided to make a hobby programming language to see what may be able to be done with it. 

## Features

- Custom syntax with unique operators and keywords
- Documentation comments using `#`, similar to Rust's style
- Lexer and parser built from scratch in Rust

## Getting Started

To run the REPL:

```sh
cargo run
```

To run a fddl script:

```sh
cargo run path/to/script.fddl
```

## Examples

```sh
func main() {
    print(`hello, world in fddl`);
}     
```

```sh
##! This is a sample module

module math {

    ### Computes the square of a number
    func square(x) => x ^ 2;
}

define $number := 5;
print(`The square of $number is ${math.square($number)}`);
```

(At least for right now.)

## License

This project is licensed under the MIT License.


---

## **Notes and Next Steps**

- [x] Added first new set of tokens and features, added the first `lexer` tests.
- [ ] `parser` module is a placeholder.
- [ ] `interpreter` module is a placeholder.
- [ ] Implement a more robust error handling mechanism instead of using `stderr`.
- [ ] Imlement string interpolation (backticks with `$variable`) 
- [ ] Continue to expand tests to cover all new syntax and features.
- [x] Made a crappy website.    

---

## **Running the Project**

Make sure your project compiles and the tests pass:

```bash
cargo build
cargo test
```