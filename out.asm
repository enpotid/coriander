global _start

_start:
    mov rax, 1
    push rax

exit:
    mov rax, 60
    mov rdi, 1
    syscall
    ret

