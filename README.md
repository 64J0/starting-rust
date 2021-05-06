# Starting RUST

This repository will be used to store some code that I'll write while learning to program in Rust Lang. To teach me the fundamentals my approach will be to use the official documentation that is available on the [project site](https://doc.rust-lang.org/book/).

Focus of the language:

> The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control. - Rust Docs.

Rust tools:

* _rustup_, command line tool for managing Rust versions and associated tools. 
* Cargo, the included dependency manager and build tool, makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers Integrated Development Environment (IDE) integration for code completion and inline error messages.

Local docs: ```rustup doc```

## Compiling and running

Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your source file.

```bash
$ rustc <program_name>.rs
```

---
Vinícius Gajo Marques Oliveira, 2021