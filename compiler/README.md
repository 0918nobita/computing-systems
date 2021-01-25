# BASIC Language Compiler

## Target

x86-64 Linux

## Requirements

- Cargo
- GNU Binutils
- NASM

## Usage

```bash
cargo run -- ../basic/hello.bas  # outputs ../basic/hello.bin
../basic/hello.bin  # => Hello, world!
```

## Build

```bash
cargo build
```

## Generate documentation

```bash
cargo doc
```

## Clean

```bash
cargo clean
```
