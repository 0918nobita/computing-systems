bits 64
global _start

section .data
    msg db 'Hello, world!', 10

section .text
_start:
    mov rdi, msg
    call printString
    mov rax, 60   ; sys_exit
    xor rdi, rdi  ; 0
    syscall

printString:
    call stringLength
    mov rdx, rax  ; length
    mov rax, 1    ; sys_write
    mov rsi, rdi  ; address
    mov rdi, 1    ; stdout
    syscall
    ret

stringLength:
    xor rax, rax
.loop:
    cmp byte[rdi+rax], 0
    je .end
    inc rax
    jmp .loop
.end:
    ret
