# http://wiki.osdev.org/64-bit_Higher_Half_Kernel_with_GRUB_2
ISO := os.iso
OUTPUT := kernel.sys
 
OBJS := boot.o main.o
 
all:	$(ISO) func.rs
 
$(ISO):	$(OUTPUT)
	cp $(OUTPUT) iso/boot
	grub-mkrescue -o $@ iso
 
$(OUTPUT): $(OBJS) linker.ld
	rustc func.rs 
	ld -nodefaultlibs -Tlinker.ld -o $@ $(OBJS) librustcode.a
 
.s.o:
	nasm -felf64 $< -o $@
 
.c.o:
	gcc -m64 -mcmodel=kernel -ffreestanding -nostdlib -mno-red-zone -c $< -o $@
 
clean:
	@rm -f $(OBJS) $(OUTPUT) iso/boot/$(OUTPUT)
