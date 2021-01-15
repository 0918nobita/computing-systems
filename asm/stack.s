; nasm -f elf64 stack.s
; ld -o stack stack.o
; ./stack

bits 64
global _start

section .data

section .text
_start:
    mov rax, 60   ; sys_exit
    xor rdi, rdi  ; 0
    syscall
