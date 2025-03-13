section .text
    global _start

_start:
    mov rax, 10
    push rax
    mov rax, 5
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 2
    push rax
    pop rbx
    pop rax
    mul rbx
    push rax
    mov rax, 50
    push rax
    mov rax, 4
    push rax
    mov rax, 1
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    pop rbx
    pop rax
    xor rdx, rdx
    div rbx
    push rax
    push QWORD [rsp + 8]
    push QWORD [rsp + 8]
    pop rbx
    pop rax
    xor rdx, rdx
    div rbx
    push rdx
    mov rax, 1
    mov rdi, 1
    mov rsi, msg1
    mov rdx, len1
    syscall
    push QWORD [rsp + 16]
    pop rax
    call print_number
    mov rax, 1
    mov rdi, 1
    mov rsi, msg2
    mov rdx, len2
    syscall
    push QWORD [rsp + 8]
    pop rax
    call print_number
    mov rax, 1
    mov rdi, 1
    mov rsi, msg3
    mov rdx, len3
    syscall
    push QWORD [rsp + 0]
    pop rax
    call print_number
    push QWORD [rsp + 16]
    push QWORD [rsp + 16]
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    push QWORD [rsp + 8]
    mov rax, 3
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    pop rbx
    pop rax
    mul rbx
    push rax
    mov rax, 2
    push rax
    pop rbx
    pop rax
    xor rdx, rdx
    div rbx
    push rax
    mov rax, 1
    mov rdi, 1
    mov rsi, msg4
    mov rdx, len4
    syscall
    push QWORD [rsp + 0]
    pop rax
    call print_number
    push QWORD [rsp + 0]
    push QWORD [rsp + 16]
    mov rax, 3
    push rax
    pop rbx
    pop rax
    mul rbx
    push rax
    push QWORD [rsp + 32]
    mov rax, 2
    push rax
    pop rbx
    pop rax
    xor rdx, rdx
    div rbx
    push rdx
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 1
    mov rdi, 1
    mov rsi, msg5
    mov rdx, len5
    syscall
    push QWORD [rsp + 0]
    pop rax
    call print_number
    mov rax, 60
    mov rdi, 0
    syscall

print_number:
    cmp rax, 0
    jne .compute
    mov rax, 1
    mov rdi, 1
    mov rsi, zero_msg
    mov rdx, zero_len
    syscall
    ret

.compute:
    xor r8, r8

.compute_loop:
    xor rdx, rdx
    mov rbx, 10
    div rbx
    add dl, '0'
    push rdx
    inc r8
    cmp rax, 0
    jne .compute_loop

.print_loop:
    pop rax
    sub rsp, 8
    mov byte [rsp], al
    mov rsi, rsp
    mov rax, 1
    mov rdi, 1
    mov rdx, 1
    syscall
    add rsp, 8
    dec r8
    cmp r8, 0
    jne .print_loop
    ret

section .data
    msg1 db 'Step 1: a = (10 + 5) * 2 = '
    len1 equ $ - msg1
    msg2 db 'Step 2: b = 50 / (4 + 1) = '
    len2 equ $ - msg2
    msg3 db 'Step 3: c = a % b = '
    len3 equ $ - msg3
    msg4 db 'Step 4: d = (a - b) * (c + 3) / 2 = '
    len4 equ $ - msg4
    msg5 db 'Final Result: e = '
    len5 equ $ - msg5
    zero_msg db '0'
    zero_len equ $ - zero_msg