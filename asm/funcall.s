; nasm -f elf64 funcall.s
; ld -o funcall funcall.o
; ./funcall

bits 64
global _start

section .data
    newline db 10
    char_a db 'A'
    char_b db 'B'

section .text
print_char:
    pop rbx     ; リターンアドレスを退避
    mov rax, 1
    mov rdi, 1
    pop rsi
    mov rdx, 1
    syscall
    push rbx    ; リターンアドレスを復旧
    ret

_start:
    push char_a
    call print_char
    push newline
    call print_char
    push char_b
    call print_char
    push newline
    call print_char
    mov rax, 60      ; sys_exit
    xor rdi, rdi     ; 0
    syscall
