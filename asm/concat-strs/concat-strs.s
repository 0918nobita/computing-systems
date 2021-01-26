bits 64
global concat_strings

%define SYS_BRK 12
%define SYS_EXIT 60

section .bss
    current_break resq 1
    length_a      resq 1
    length        resq 1

section .text
concat_strings:
    push rsi
    push rdi

    ; 結合後の文字数を求める
    call string_length
    mov [length_a], rax
    mov rcx, rax
    call string_length
    add rcx, rax
    mov [length], rcx

    ; 現在のブレーク値を取得する
    mov rax, SYS_BRK
    xor rdi, rdi
    syscall

    ; (結合後の文字列の文字数) + 1 バイト分ヒープ領域を確保する
    mov [current_break], rax
    add rax, [length]
    add rax, 1
    mov rdi, rax
    mov rax, SYS_BRK
    syscall

    ; 2つの文字列を順番に [current_break] にコピーする
    pop rdi
    xor rcx, rcx
.loop_a:
    mov al, [rdi+rcx]
    mov [current_break+rcx], al
    inc rcx
    cmp rcx, [length_a]
    jl .loop_a
    pop rdi
    xor rbx, rbx
.loop_b:
    mov al, [rdi+rbx]
    mov [current_break+rcx], al
    inc rcx
    inc rbx
    cmp rcx, [length]
    jl .loop_b

    ; 終端にヌル文字を書き込む
    mov rax, [length]
    add rax, 1
    mov byte [current_break+rax], 0

    mov rax, current_break
    ret

string_length:
    xor rax, rax
.loop:
    cmp byte[rdi+rax], 0
    je .end
    inc rax
    jmp .loop
.end:
    ret
