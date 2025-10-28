org 0x7C00
bits 16

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov [boot_drive], dl 
    mov ss, ax
    mov sp, 0x7C00
    sti

    mov ax, 0x0003
    int 0x10

    mov si, msg_kernel
    call print
    call nl
    mov si, msg_check
    call print
    call nl

    mov si, msg_disk
    call print
    mov dl, [boot_drive]
    mov ah, 0x00
    int 0x13
    jc .disk_fail
    call ok
    jmp .disk_done
.disk_fail:
    call fail
.disk_done:
    
    mov si, msg_mem
    call print
    int 0x12
    test ax, ax
    jle .mem_fail
    call ok
    jmp .mem_done
.mem_fail:
    call fail
.mem_done:
    call nl

    mov si, logo1
    call print
    call nl
    mov si, logo2
    call print
    call nl
    mov si, logo3
    call print
    call nl
    call nl

    mov si, msg_load
    call print
    
    xor ax, ax
    mov es, ax
    mov bx, 0x7E00
    
    mov ah, 0x02
    mov al, 4
    mov ch, 0x00
    mov cl, 0x02
    mov dh, 0x00
    mov dl, [boot_drive]
    
    int 0x13
    jc load_err
    
    call nl
    mov si, msg_ok
    call print
    call nl
    call nl
    
    ; Jump to installer
    jmp 0x0000:0x7E00

; Error handler
load_err:
    call nl
    mov si, msg_err
    call print
    call nl
    jmp halt_forever

; Print routines
print:
    pusha
.loop:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp .loop
.done:
    popa
    ret

nl:
    push ax
    mov ah, 0x0E
    mov al, 13
    int 0x10
    mov al, 10
    int 0x10
    pop ax
    ret

ok:
    push si
    mov si, msg_ok
    call print
    pop si
    ret

fail:
    push si
    mov si, msg_fail
    call print
    pop si
    ret

halt_forever:
    cli
.loop:
    hlt
    jmp .loop ; check discord

boot_drive  db 0
msg_kernel  db "UltraOS",0
msg_check   db "Checking...",0 
msg_disk    db "Disk: ",0
msg_mem     db "Memory: ",0
msg_ok      db "[OK]",0
msg_fail    db "[FAIL]",0
msg_load    db "Loading installer...",0
msg_err     db "[ERR] Cannot load installer",0
logo1       db " | | | | | |_ _ __ ___| |  | | (___  ",0
logo2       db " | | | | | __| '__/ _ \ |  | |\___ \ ",0
logo3       db " | |_| | | |_| | |  __/ |__| |____) |",0

times 510-($-$$) db 0
dw 0xAA55 ; i see you buddy hehehehehehe