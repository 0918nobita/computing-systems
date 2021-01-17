bits 64
global _start

%define O_RDONLY 0
%define PROT_READ 1
%define MAP_PRIVATE 2

section .data
    filename db 'input.txt'

section .text
_start:
    mov rax, 2  ; sys_open
    mov rdi, filename
    mov rsi, O_RDONLY
    mov rdx, 0
    syscall ; -> rax に、openしたファイルのディスクリプタが格納される

    mov r8, rax           ; マップされるファイルのディスクリプタ
    mov rax, 9            ; sys_mmap
    mov rdi, 0            ; マップ先はOSに選んでもらう
    mov rsi, 4096         ; ページサイズ
    mov rdx, PROT_READ    ; 新しいメモリ領域は readonly とマークされる
    mov r9, 0             ; ファイル内のオフセット
    mov r10, MAP_PRIVATE  ; ページの共有なし
    syscall

    mov rdi, rax
    call printString

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
