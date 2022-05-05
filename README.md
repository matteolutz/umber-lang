<p align="center">
<img height="100" src="./assets/img/logo.png">

<h1 align="center">
    umber-lang
</h1>
<p align="center">
    A Rust compiled language by <a href="https://matteolutz.de">Matteo Lutz</a>
</p>

</p>

<br />

## Table of contents

- [What is Umber?](#what-is-umber)
- [How to use it](#how-to-use-it)

## What is Umber

Umber is an compiled (soon multiparadigm) programming language, implemented in Rust. It's currently _WIP_ so please use it on your own risk.  
Future commits **WILL** probably contain **BREAKING CHANGES**.

## How to use it

### Build

First you have to clone the repository and cd into it.

```
git clone https://github.com/matteolutz/umber-lang.git
cd umber-lang
```

Now you can run the binary crate with `cargo run`.  
To just build the crate, use `cargo build`.

### Compile a file

```
cargo run <FILENAME>.ub
```

Umber now created a new assembly file with the same name as the input file, but with the extension `.asm`.
Currently, you can only build and link the assembly file on x64-86 unix systems. To do this, just run:

```
nasm -f elf64 <FILENAME>.asm
ld <FILENAME>.o -o <FILENAME>
```

You can run the final binary with `./<FILENAME>`.
