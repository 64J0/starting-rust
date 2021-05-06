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

## Package manager: Cargo

To create a new project with a simple scaffold we can use the command ```cargo new <project_name>```. This command creates a new folder with the <project_name>/ and add a src/ subfolder with main.rs and a **Cargo.toml** file.

Inspecting this **Cargo.toml** file I can say that its structure is similiar to the **package.json** for **Node.js** projects.

> This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format. - Rust Docs.

> In Rust, packages of code are referred to as crates. - Rust Docs.

> Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. - Rust Docs.

To build a Cargo project we need to open the directory of that Cargo project in the terminal and type ```cargo build```. Then this tool will create a target/ folder inside the project tree at the same level as the src/ folder and a Cargo.lock file.

About the Cargo.lock:

> This file keeps track of the exact versions of dependencies in your project. - Rust Docs.

In other words it's the same thing as the package-lock.json for Node.js.

There's a command to build and run the Cargo project directly that is ```cargo run```.

There's also a command to check if the code compiles but without compiling it after all: ```cargo check```. This approach is good because this command runs much faster than the ```cargo build``` because it skips the step of producing an executable.

### Build for release:

> When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release. - Rust Docs.

---
Vinícius Gajo Marques Oliveira, 2021