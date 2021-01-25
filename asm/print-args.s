bits 64
global _start

section .data
    newline db 10

section .text
_start:
    pop rcx
.loop:
    dec rcx
    pop rdi
    push rcx
    call printString
    call printNewline
    pop rcx
    test rcx, rcx
    jnz .loop
    mov rax, 60
    xor rdi, rdi
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

printNewline:
    mov rax, 1        ; sys_write
    mov rdi, 1        ; stdout
    mov rsi, newline  ; address
    mov rdx, 1        ; length
    syscall
    ret
