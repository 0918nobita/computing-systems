bits 64
global _start

section .text
_start:
    mov rax, 60
    pop rdi
    syscall
