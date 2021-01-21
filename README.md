# BASIC

(WIP) Language implementation in Rust

Target: x86-64 Linux

## Requirements

- GNU Binutils
- NASM

## Compiler

```bash
cd compiler
cargo run -- ../basic/hello.bas
../basic/hello.bin

cargo run -- ../basic/variables.bas
../basic/variables.bin
```

or

```bash
cargo run --bin compiler -- ./basic/hello.bas
./basic/hello.bin

cargo run --bin compiler -- ./basic/variables.bas
./basic/variables.bin
```

## `t2b` (Text to Binary) Tool

```bash
cd t2b
echo eb 00 | cargo run | xxd
00000000: eb00                                     ..
```
