bits 64
global _start

%define SYS_WRITE 1
%define SYS_BRK 12
%define SYS_EXIT 60

section .data
    newline db 10
    codes db '0123456789ABCDEF'

section .text
_start:
    ; 現在のブレーク値を取得
    mov rax, SYS_BRK
    xor rdi, rdi
    syscall

    ; 取得したブレーク値をデバッグ出力
    push rax  ; base = rax
    push rax
    mov rdi, rax
    call printInt
    call printNewline

    ; インクリメントされたブレーク値を brk システムコールで指定してヒープ領域を確保
    pop rax
    mov rdi, rax
    add rdi, 5
    mov rax, SYS_BRK
    syscall

    ; 確保した後のブレーク値をデバッグ出力
    push rax
    mov rdi, rax
    call printInt
    call printNewline

    ; 確保したヒープ領域にデータを書き込む
    pop rax
    mov byte[rax],   0x41
    mov byte[rax+1], 0x42
    mov byte[rax+2], 0x43
    mov byte[rax+3], 0x0A
    mov byte[rax+4], 0x00
    mov rdi, rax
    call printString

    ; もともとのブレーク値を brk システムコールで指定してヒープ領域を解放
    mov rax, SYS_BRK
    pop rdi      ; rdi = base
    syscall

    ; 解放した後のブレーク値をデバッグ出力
    mov rdi, rax
    call printInt
    call printNewline

    mov rax, SYS_EXIT
    xor rdi, rdi
    syscall

printString:
    call stringLength
    mov rdx, rax  ; length
    mov rax, SYS_WRITE
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
    mov rax, SYS_WRITE
    push rcx               ; i をスタックに退避
    syscall
    pop rcx                ; i を復元
    pop rax                ; N を復元
    test rcx, rcx
    jnz .loop              ; i が 0 でなければジャンプする
    ret

printNewline:
    mov rax, SYS_WRITE
    mov rdi, 1        ; stdout
    mov rsi, newline  ; address
    mov rdx, 1        ; length
    syscall
    ret
