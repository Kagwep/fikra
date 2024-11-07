global _start
_start:
    mov rax, 5
    push rax
    mov rax, 1
    push rax
    mov rax, 3
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    mov rax, 2
    push rax
    pop rbx
    pop rax
    div rbx
    push rax
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    mov rax, 2
    push rax
    mov rax, 1
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    push QWORD [rsp + 0]
    mov rax, 60
    pop rdi
    syscall
    add rsp, 8
    push QWORD [rsp + 0]
    mov rax, 8
    push rax
    pop rax
    pop rbx
    add rax, rbx
    push rax
    push QWORD [rsp + 0]
    mov rax, 60
    pop rdi
    syscall
    mov rax, 60
    mov rdi, 0
    syscall
