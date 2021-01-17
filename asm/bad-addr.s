bits 64
global _start

section .text
_start:
    mov rax, [0x400000-1]
    mov rax, 60
    xor rdi, rdi
    syscall
