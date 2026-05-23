; multiboot2.asm – boot header and entry point
section .multiboot
align 8
    dd 0xe85250d6                 ; magic
    dd 0                          ; architecture (i386)
    dd header_end - header_start  ; header length
    dd -(0xe85250d6 + 0 + (header_end - header_start)) ; checksum

header_start:
    ; framebuffer tag (optional)
    dw 5                          ; type = framebuffer
    dw 1                          ; flags
    dd 8 + 8 + 4                  ; size
    dd 0                          ; width (0 = prefer)
    dd 0                          ; height
    dd 0                          ; depth
    ; end tag
    dw 0                          ; type
    dw 0                          ; flags
    dd 8                          ; size
header_end:

section .text
global _start
extern kernel_main                ; defined in Navescript
_start:
    mov esp, stack_top
    push eax                      ; multiboot magic
    push ebx                      ; multiboot info
    call kernel_main
    cli
    hlt
    jmp $

section .bss
align 16
stack_bottom:
    resb 16384                    ; 16KB stack
stack_top:
