; nasm -f macho64 hello.s
; ld -lSystem -o hello.bin hello.o
; ./hello.bin

bits 64
global _main

%define FD_STDOUT 1

%define SYS_EXIT  0x2000001
%define SYS_WRITE 0x2000004

section .data
    msg db 'Hello, world!', 10

section .text
_main:
    mov rax, SYS_WRITE
    mov rdi, FD_STDOUT
    mov rsi, msg
    mov rdx, 14
    syscall

    mov rax, SYS_EXIT
    xor rdi, rdi  ; exit code
    syscall
