global _start
_start:
    mov rax, 1
    push rax
    mov rax, 2
    push rax
    pop rbx
    pop rax
    mul rbx
    push rax
    push QWORD [rsp + 0]
    mov rax, 60
    pop rdi
    syscall
    mov rax, 60
    mov rdi, 0
    syscall
