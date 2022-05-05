<p align="center">
<img width="100" src="data:image/svg+xml,%3Csvg id='Ebene_1' data-name='Ebene 1' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 188.88 188.88'%3E%3Crect width='188.88' height='188.88' style='fill:%23cf4f31'/%3E%3Cpath d='M308.55,319.93V192c-6.68,0-13-.25-19.3.14a9.51,9.51,0,0,0-6.12,3q-12.93,15.55-25.31,31.56c-2.69,3.47-4.5,3.49-7.21,0C242.34,216,233.85,205.53,225.37,195c-1-1.21-2.4-2.83-3.69-2.89-5.91-.31-11.84-.14-17.82-.14V208.1a29.74,29.74,0,0,0,4.37,0c5-.8,8.27,1.21,11.23,5.26,5.3,7.24,11.89,13.64,16.59,21.23,5.75,9.31,13.33,12.68,23.64,10.67,2.19-.42,4.77-1.43,6.14-3C272,235,277.74,227.42,283.66,220c2.62-3.29,5.27-6.55,7.91-9.82l1.09.44V319.93ZM247.35,265c0,8.66.09,16.79,0,24.92a13.66,13.66,0,0,1-9,13.13c-9.1,3.38-18.33-3.16-18.47-13.4-.2-13.32,0-26.65-.16-40,0-1.41-1.26-3.94-2.11-4-4.56-.4-9.18-.19-14.16-.19,0,14.87,0,29,0,43.13a40.12,40.12,0,0,0,.34,5.48,29.91,29.91,0,0,0,28.5,26c14.74.47,28.37-9.25,30.37-23.58,1.43-10.22.27-20.81.27-31.51Z' transform='translate(-161.56 -161.56)' style='fill:%236e260e'/%3E%3Cpath d='M308.55,319.93H292.66V210.62l-1.09-.44c-2.64,3.27-5.29,6.53-7.91,9.82-5.92,7.42-11.69,15-17.83,22.19-1.37,1.61-3.95,2.62-6.14,3-10.31,2-17.89-1.36-23.64-10.67-4.7-7.59-11.29-14-16.59-21.23-3-4.05-6.25-6.06-11.23-5.26a29.74,29.74,0,0,1-4.37,0V192c6,0,11.91-.17,17.82.14,1.29.06,2.71,1.68,3.69,2.89,8.48,10.49,17,21,25.24,31.63,2.71,3.49,4.52,3.47,7.21,0q12.42-16,25.31-31.56a9.51,9.51,0,0,1,6.12-3c6.29-.39,12.62-.14,19.3-.14Z' transform='translate(-161.56 -161.56)' style='fill:%236e260e'/%3E%3Cpath d='M247.35,265h15.59c0,10.7,1.16,21.29-.27,31.51-2,14.33-15.63,24.05-30.37,23.58a29.91,29.91,0,0,1-28.5-26,40.12,40.12,0,0,1-.34-5.48c0-14.13,0-28.26,0-43.13,5,0,9.6-.21,14.16.19.85.07,2.09,2.6,2.11,4,.16,13.33,0,26.66.16,40,.14,10.24,9.37,16.78,18.47,13.4a13.66,13.66,0,0,0,9-13.13C247.44,281.78,247.35,273.65,247.35,265Z' transform='translate(-161.56 -161.56)' style='fill:%236e260e'/%3E%3C/svg%3E">

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
