section .data
    block_index     dq 1
    block_timestamp dq 1795348260
    block_data      db "Alice->Bob:10", 0, 0, 0
    prev_hash       times 32 db 0x00
    nonce           dq 0

section .bss
    current_hash    resb 32

section .text
    global _start

_start:
mine_loop:
    inc qword [nonce]

    mov rdi, block_index
    mov rsi, 72
    mov rdx, current_hash
    call sha256_hash
    
    mov ax, [current_hash]
    cmp ax, 0
    jne mine_loop

    mov rax, 60
    xor rdi, rdi
    syscall

sha256_hash:
    ret
