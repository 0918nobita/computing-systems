bits 64
global _start

section .data
    codes db '0123456789ABCDEF'

section .text
_start:
    sub rsp, 16
    mov rbp, rsp

    mov qword[rbp], 3
    mov qword[rbp+8], 4

    mov rdi, [rbp]
    add rdi, [rbp+8]
    call printInt

    mov rax, 60
    xor rdi, rdi
    syscall

printInt:
    mov rax, rdi           ; 表示する数値 N
    mov rdi, 1             ; ディスクリプタ (1 = stdout)
    mov rdx, 1             ; 文字列のバイト数
    mov rcx, 64            ; カウンタ i (64 → 0)
.loop:
    push rax               ; N の現在の値をスタックに退避
    sub rcx, 4             ; i = i - 4
    sar rax, cl            ; rax に残った N を i ビットだけ算術右シフト
    and rax, 0xf           ; rax に rax と 0xf の論理積を格納し、16進の1桁を得る
    lea rsi, [codes + rax] ; 文字列のアドレス
    mov rax, 1             ; sys_write
    push rcx               ; i をスタックに退避
    syscall
    pop rcx                ; i を復元
    pop rax                ; N を復元
    test rcx, rcx
    jnz .loop              ; i が 0 でなければジャンプする
    ret
