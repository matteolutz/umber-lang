<p align="center">
<a href="https://github.com/matteolutz/umber-lang"><img height="100" src="./assets/img/logo.png"></a>
    
<h1 align="center">
    Umber
</h1>
<h5 align="center">
<i>Spoken</i>: [ˈʌmbəʳ]
</h5>
<p align="center">
    A compiled language by <a href="https://matteolutz.de">Matteo Lutz</a> implemented in Rust
</p>
</p>

<br />

## Table of contents

- [What is Umber?](#what-is-umber)
- [How to use it](#how-to-use-it)

## What is Umber

Umber is a compiled (soon multiparadigm) programming language, with it's current compiler implemented in Rust. It's currently _WIP_ so please use it at your own risk.  
Future commits **WILL** for sure contain **BREAKING CHANGES**.

## How to use it

### Build

First you have to clone the repository and cd into it.

```shell
git clone https://github.com/matteolutz/umber-lang.git
cd umber-lang
```

Now you can run the binary crate with `cargo run`.  
To just build the crate, use `cargo build`.

### Compile a file

To compile a file it is important to have the NASM-Assembler installed. Otherwise the compiler won't be able to build a binary from the generated NASM-Assembly.  
Then just run:

```shell
cargo run com <FILENAME>.ub
```
.  
The Umber compiler now created a directory called `build` in the current directory containing the generated assembly, object file and binary.  
To specify include paths, you can use the `--include` (or `-i`) flag.

````shell
cargo run com <FILENAME>.ub -i <PATH>
````
or, if you want to use more than one include path:
````shell
cargo run com <FILENAME>.ub -i <PATH1>;<PATH2>;<PATH3>;...
````

.  
To pass additional command line arguments for the linker you can use the `--linker` (or `-l`) flag. Here an example:

````shell
cargo run com <FILENAME>.ub -l"-dynamic-linker /lib64/ld-linux-x86-64.so.2 -lc"
````

.  
For additional command line arguments that are passed to the NASM-Assembler the syntax is the same as for the linker. Just use the `--asm` (or `-a`) flag.

By saying
````shell
cargo run help
````
the Umber compiler will print a help message to the console. If you ever get stuck, just run `cargo run help` again.

To install the compiler globally to your system first build the crate in "Release" mode (tells the Rust compiler to use all possible optimization) with `cargo build -r` and then just copy the Rust binaries living in `target/release` into your system's `bin` directory.  
For example:
````shell
cargo build -r
cp target/release/umber /usr/local/bin/umber
````
.  
Now you can
```shell
umber com
umber help
...
```
.  
You can use all the options shown above by just replace the `cargo run` with `umber`.