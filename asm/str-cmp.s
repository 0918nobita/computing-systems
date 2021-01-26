bits 64
global _start

%define EXIT_FAILURE 1

%define FD_STDOUT 1
%define FD_STDERR 2

%define SYS_WRITE 1
%define SYS_EXIT 60

section .data
    eq_msg     db 'Equal', 10, 0
    not_eq_msg db 'Not Equal', 10, 0
    err_msg    db 'Error: 2nd & 3rd command-line arguments are required', 10, 0

section .bss
    str_a resq 1
    str_b resq 1
    len_a resq 1

section .text
_start:
    pop rax
    cmp rax, 3
    jl .error

    add rsp, 8  ; スタックの先頭要素を削除する

    ; 1つ目の文字列を [str_a] に書き込み、[len_a] にその文字数を格納する
    pop rdi
    mov [str_a], rdi
    call stringLength
    mov [len_a], rax

    ; 2つ目の文字列を [str_b] に書き込み、rax にその文字数を格納する
    pop rdi
    mov [str_b], rdi
    call stringLength

    ; 文字数を比較する
    mov rdi, [len_a]
    cmp rdi, rax
    jne .not_equal

    ; 終端の文字から先頭文字まで1文字ずつ比較する
    mov rcx, rdi
.loop:
    dec rcx
    mov rax, [str_a]
    mov al, [rax+rcx]
    mov rdi, [str_b]
    mov dil, [rdi+rcx]
    cmp al, dil
    jne .not_equal
    test rcx, rcx
    jnz .loop

    mov rdi, eq_msg
    call printString

    mov rax, SYS_EXIT
    xor rdi, rdi
    syscall

.error:
    mov rdi, err_msg
    call stringLength
    mov rdx, rax
    mov rax, SYS_WRITE
    mov rdi, FD_STDERR
    mov rsi, err_msg
    syscall
    mov rax, SYS_EXIT
    mov rdi, EXIT_FAILURE
    syscall

.not_equal:
    mov rdi, not_eq_msg
    call printString
    mov rax, SYS_EXIT
    xor rdi, rdi
    syscall

printString:
    call stringLength
    mov rdx, rax  ; length
    mov rax, SYS_WRITE
    mov rsi, rdi  ; address
    mov rdi, FD_STDOUT
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
