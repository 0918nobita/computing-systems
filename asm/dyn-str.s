bits 64
global _start

section .text
_start:
    mov rax, 12  ; sys_brk
    xor rdi, rdi
    syscall

    push rax
    mov rdi, rax
    add rdi, 5
    mov rax, 12  ; sys_brk
    syscall

    mov byte[rax],   0x41
    mov byte[rax+1], 0x42
    mov byte[rax+2], 0x43
    mov byte[rax+3], 0x0A
    mov byte[rax+4], 0x00
    mov rdi, rax
    call printString

    mov rax, 12  ; sys_brk
    pop rdi
    syscall

    mov rax, 60  ; sys_exit
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
