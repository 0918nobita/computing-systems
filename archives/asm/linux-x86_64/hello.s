bits 64
global _start

section .data
    msg db  'hello, world', 0x0A

section .text
_start:
    mov rcx, 3    ; カウンタの初期値を設定
.loop:
    dec rcx       ; カウンタを減らす
    mov rax, 1    ; sys_write
    mov rdi, 1    ; stdout
    mov rsi, msg  ; address
    mov rdx, 13  ; length
    push rcx      ; syscall が rcx を書き換えるので値をスタックに退避する
    syscall
    pop rcx       ; 退避していた値を戻す
    test rcx, rcx ; rcx が 0 ならゼロフラグが立つ
    jnz .loop     ; ゼロフラグが立っていなければジャンプする

    mov rax, 60   ; sys_exit
    xor rdi, rdi  ; 0
    syscall
