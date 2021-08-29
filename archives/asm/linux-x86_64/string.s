bits 64
global _start

section .data
    test_string db "abcdef", 0

section .text
strlen:
    xor rax, rax  ; len = 0
.loop:
    cmp byte[rdi+rax], 0
    je .end
    inc rax
    jmp .loop
.end:
    ret

_start:
    mov rdi, test_string
    call strlen
    mov rdi, rax
    mov rax, 60   ; sys_exit
    syscall
