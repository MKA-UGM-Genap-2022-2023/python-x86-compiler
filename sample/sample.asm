section .text
  global _start

_start:
  mov rax, 5
  mov rbx, 3
  add rax, rbx

  mov rax, 60
  xor rdi, rdi
  syscall
