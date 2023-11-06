.global _start
.intel_syntax noprefix

_start:
  // sys write
  mov rax, 1
  mov rdi, 1
  lea rsi, [hello_world]
  mov rdx, 13
  syscall

  // sys exit
  mov rdi, 8
  mov rsi, rdi
  mov rax, 60
  syscall

hello_world:
  .asciz "Hello, World!\n"
