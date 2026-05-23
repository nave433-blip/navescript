; x86_64.asm – low‑level helpers
section .text
global load_gdt
global load_idt
global set_tss_rsp
global enable_sse
global outb, inb

; load_gdt(gdtr_ptr)
load_gdt:
    lgdt [rdi]
    ret

; load_idt(idtr_ptr)
load_idt:
    lidt [rdi]
    ret

; set_tss_rsp(rsp)
set_tss_rsp:
    mov rsp, rdi
    ret

enable_sse:
    mov rax, cr0
    and ax, 0xFFFB
    or ax, 0x2
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9
    mov cr4, rax
    ret

outb:
    mov dx, di
    mov al, sil
    out dx, al
    ret

inb:
    mov dx, di
    in al, dx
    ret
