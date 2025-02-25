section .text
    global _start

_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg1
    mov rdx, len1
    syscall
    mov rax, 10
    push rax
    mov rax, 4
    push rax
    push QWORD [rsp + 8]
    push QWORD [rsp + 8]
    pop rbx
    pop rax
    mul rbx
    push rax
    push QWORD [rsp + 0]
    mov rax, 3
    push rax
    pop rbx
    pop rax
    xor rdx, rdx
    div rbx
    push rdx
    push QWORD [rsp + 24]
    push QWORD [rsp + 24]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 16]
    push QWORD [rsp + 40]
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    push QWORD [rsp + 8]
    push QWORD [rsp + 8]
    pop rbx
    pop rax
    mul rbx
    push rax
    push QWORD [rsp + 0]
    push QWORD [rsp + 32]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 0
    push rax
    push QWORD [rsp + 8]
    push QWORD [rsp + 8]
    push QWORD [rsp + 8]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 16]
    push QWORD [rsp + 16]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 1
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 24]
    push QWORD [rsp + 24]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 2
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 32]
    push QWORD [rsp + 32]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 40]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 32]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 1
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 40]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    push QWORD [rsp + 32]
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 2
    push rax
    pop rbx
    pop rax
    add rax, rbx
    push rax
    mov rax, 60
    pop rdi
    syscall
    mov rax, 60
    mov rdi, 0
    syscall

section .data
    msg1 db 'hello world!', 0xA
    len1 equ $ - msg1
