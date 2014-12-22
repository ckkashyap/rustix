[BITS 32]
[SECTION .mbhdr]
[EXTERN _loadStart]
[EXTERN _loadEnd]
[EXTERN _bssEnd]

 
ALIGN 8
MbHdr:
	; Magic
	DD	0xE85250D6
	; Architecture
	DD	0
	; Length
	DD	HdrEnd - MbHdr
	; Checksum
	DD	-(0xE85250D6 + 0 + (HdrEnd - MbHdr))
 
	;
	; Tags
	;
 
	; Sections override
	DW	2, 0
	DD	24
	DD	MbHdr
	DD	_loadStart
	DD	_loadEnd
	DD	_bssEnd
 
	; Entry point override
	DW	3, 0
	DD	12
	DD	EntryPoint
        DD      0           ; align next tag to 8 byte boundary
 
	; End Of Tags
	DW	0, 0
	DD	8
 
	; Hdr End Mark
mboot:
        %define MULTIBOOT1_HEADER_MAGIC  0x1BADB002
        %define MULTIBOOT1_HEADER_FLAGS  0x00010000
        %define MULTIBOOT1_CHECKSUM      -(MULTIBOOT1_HEADER_MAGIC + MULTIBOOT1_HEADER_FLAGS)
        
        ; This is the GRUB Multiboot header. A boot signature
        dd MULTIBOOT1_HEADER_MAGIC
        dd MULTIBOOT1_HEADER_FLAGS
        dd MULTIBOOT1_CHECKSUM
        dd mboot
        
        dd _loadStart
        dd _loadEnd
        dd _bssEnd
        dd EntryPoint
        
	; VBE related
        dd 0    ; Mode
        dd 0    ; Width 
        dd 0    ; Height
        dd 32   ; Depth



HdrEnd:


[SECTION .boot]
[GLOBAL EntryPoint]
[EXTERN Stack]
EntryPoint:
	mov eax, Gdtr1
	lgdt [eax]
 
	push 0x08
	push .GdtReady
	retf
 
.GdtReady:
	mov eax, 0x10
	mov ds, ax
	mov ss, ax
	mov esp, Stack
 
	call SetupPagingAndLongMode
 
	mov eax, Gdtr2
	lgdt [Gdtr2]
 
	push 0x08
	push .Gdt2Ready
	retf
 
[BITS 64]
[EXTERN cmain]
.Gdt2Ready:
	mov eax, 0x10
	mov ds, ax
	mov es, ax
	mov ss, ax
 
	mov rsp, Stack + 0xFFFFFFFF80000000
 
	; If you later decide to unmap the lower zone, you will have an invalid Gdt if you're still using Gdtr2
	mov rax, Gdtr3
	lgdt [rax]
 
	mov rax, cmain
	call rax
	cli
	jmp $


global memset
memset:
        mov rax, rsi
        mov rcx, rdx
        rep stosb
        ret

; DUMMY START

global memcpy
memcpy:
        ret
global pow
pow:
        ret
global memcmp
memcmp:
        ret
global fmod
fmod:
        ret
global fmodf
fmodf:
        ret
global log10
log10:
        ret
global log2
log2:
        ret
global log
log:
        ret
global exp
exp:
        ret
global exp2
exp2:
        ret
global fma
fma:
        ret
global trunc
trunc:
        ret
global round
round:
        ret
global ceil
ceil:
        ret
global floor
floor:
        ret
global log10f
log10f:
        ret
global log2f
log2f:
        ret
global expf
expf:
        ret
global exp2f
exp2f:
        ret
global powf
powf:
        ret
global logf
logf:
        ret
global roundf
roundf:
        ret
global fmaf
fmaf:
        ret
global truncf
truncf:
        ret

global ceilf
ceilf:
        ret

global floorf
floorf:
        ret
; DUMMY END



[BITS 32]
[EXTERN Pml4]
[EXTERN Pdpt]
[EXTERN Pd]
SetupPagingAndLongMode:
	mov eax, Pdpt
	or eax, 1
	mov [Pml4], eax
	mov [Pml4 + 0xFF8], eax
 
	mov eax, Pd
	or eax, 1
	mov [Pdpt], eax
	mov [Pdpt + 0xFF0], eax
 
	mov dword [Pd], 0x000083
	mov dword [Pd + 8], 0x200083
	mov dword [Pd + 16], 0x400083
	mov dword [Pd + 24], 0x600083
 
	; Load CR3 with PML4
	mov eax, Pml4
	mov cr3, eax
 
	; Enable PAE
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax
 
	; Enable Long Mode in the MSR
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr
 
	; Enable Paging
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax
 
	ret

TmpGdt:
	DQ	0x0000000000000000
	DQ	0x00CF9A000000FFFF
	DQ	0x00CF92000000FFFF
	DQ	0x0000000000000000
	DQ	0x00A09A0000000000
	DQ	0x00A0920000000000
 
Gdtr1:
	DW	23
	DD	TmpGdt
 
Gdtr2:
	DW	23
	DD	TmpGdt + 24
	DD	0
 
Gdtr3:
	DW	23
	DQ	TmpGdt + 24 + 0xFFFFFFFF80000000




