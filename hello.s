; nasm -f elf64 hello.s
; ld -o hello hello.o
; ./hello

bits 64
section .text
global _start

_start:
    mov rax, 1   ; sys_write
    mov rdi, 1   ; stdout
    mov rsi, msg ; address
    mov rdx, len ; length
    syscall

    mov rax, 60  ; sys_exit
    xor rdi, rdi ; 0
    syscall

section .data
    msg db  'hello, world', 0x0A
    len equ $ - msg
