# BASIC

(WIP) Language implementation in Rust

Target: x86_64 Linux

```bash
cargo run -- ./basic/hello.bas
nasm -f elf64 out.s
ld -o out out.o
./out
```
