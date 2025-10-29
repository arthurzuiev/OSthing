; ============================
; UltraOS Mode Switcher
; ============================

org 0x7E00
bits 16

start:
    cli
    mov ax, 0x0003
    int 0x10

    mov si, msg_title
    call print
    call nl
    call nl

    mov si, msg_option1
    call print
    call nl
    mov si, msg_option2
    call print
    call nl
    call nl
    mov si, msg_prompt
    call print

.wait_key:
    mov ah, 0
    int 0x16
    cmp al, '1'
    je normal_boot
    cmp al, '2'
    je enter_64bit
    mov ah, 0x0E
    mov al, 7
    int 0x10
    jmp .wait_key

; =====================================
; Normal boot: reboot back to bootloader
; =====================================
normal_boot:
    call nl
    call nl
    mov si, msg_reboot
    call print
    call nl

    mov cx, 0xFFFF
.delay:
    loop .delay

    mov al, 0xFE
    out 0x64, al
    hlt
    jmp $

; =====================================
; Enter 64-bit mode
; =====================================
enter_64bit:
    call nl
    call nl
    mov si, msg_entering
    call print
    call nl

    call enable_a20
    lgdt [gdt_descriptor]

    ; === Load ui.bin (C kernel interface) ===
    mov ah, 0x02
    mov al, 8
    mov ch, 0
    mov cl, 6
    mov dh, 0
    xor dl, dl
    mov bx, 0x1000
    mov es, bx
    xor bx, bx
    int 0x13
    jc disk_error

    ; === Enter protected mode ===
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 0x08:protected_mode

; =====================================
; Simple text output
; =====================================
print:
    pusha
.print_loop:
    lodsb
    or al, al
    jz .print_done
    mov ah, 0x0E
    int 0x10
    jmp .print_loop
.print_done:
    popa
    ret

nl:
    pusha
    mov ah, 0x0E
    mov al, 13
    int 0x10
    mov al, 10
    int 0x10
    popa
    ret

enable_a20:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

disk_error:
    mov si, msg_disk_err
    call print
    jmp hang

hang:
    cli
    hlt
    jmp hang

; =====================================
; Protected mode setup
; =====================================
bits 32
protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000

    mov eax, cr4
    or eax, 0x20
    mov cr4, eax

    mov edi, 0x1000
    mov cr3, edi
    xor eax, eax
    mov ecx, 4096
    rep stosd
    mov edi, cr3

    mov dword [edi], 0x2003
    mov dword [0x2000], 0x3003
    mov dword [0x3000], 0x00000083

    mov ecx, 0xC0000080
    rdmsr
    or eax, 0x100
    wrmsr

    mov eax, cr0
    or eax, 0x80000001
    mov cr0, eax

    lgdt [gdt64_descriptor]
    jmp 0x08:long_mode

; =====================================
; 64-bit Long Mode
; =====================================
bits 64
long_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rdi, 0xB8000
    mov rsi, msg_success
    call print_64

    ; Jump to C interface (ui.bin)
    mov rax, 0x10000
    jmp rax

    jmp hang

print_64:
    lodsb
    test al, al
    jz .done
    mov ah, 0x2F
    stosw
    jmp print_64
.done:
    ret

; =====================================
; Data
; =====================================
msg_title     db "=== UltraOS Boot Menu ===",0
msg_option1   db "1. Normal Boot (Reboot)",0
msg_option2   db "2. Enter 64-bit UI",0
msg_prompt    db "Select option: ",0
msg_reboot    db "Rebooting...",0
msg_entering  db "Loading 64-bit kernel...",0
msg_success   db "64-bit mode active! Launching UI...",0
msg_disk_err  db "Disk read error!",0

; =====================================
; GDTs
; =====================================
align 8
gdt_start:
    dq 0
    dq 0x00CF9A000000FFFF
    dq 0x00CF92000000FFFF
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

align 8
gdt64_start:
    dq 0
    dq 0x00209A0000000000
    dq 0x0000920000000000
gdt64_end:

gdt64_descriptor:
    dw gdt64_end - gdt64_start - 1
    dd gdt64_start

times 2048-($-$$) db 0
