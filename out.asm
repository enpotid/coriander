global _start
_start:
    mov rax, 1
    push rax
    mov rax, 2
    push rax
    mov rax, 3
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    push QWORD [rsp + 0]
    mov rax, 8
    push rax
    mov rax, 4
    push rax
    mov rax, 5
    push rax
    mov rax, 1
    push rax
    push QWORD [rsp + 40]
    mov rax, 2
    push rax
    mov rax, 6
    push rax
    mov rax, 9
    push rax
    mov rax, 10
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    push QWORD [rsp + 0]
    mov rax, 3
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    mov rax, 60
    pop rdi
    syscall
    mov rax, 60
    mov rdi, 0
    syscall
