# http://wiki.osdev.org/64-bit_Higher_Half_Kernel_with_GRUB_2
ISO := os.iso
 
all:
	ld -nodefaultlibs -Tlinker.ld -o kernel.sys ../kernel/main.o ../boot/x86_64/boot.o  ../kernel/libkernel.a
	cp kernel.sys iso/boot
	grub-mkrescue -o $(ISO) iso

clean:
	@rm -f $(ISO) iso/boot/kernel.sys kernel.sys
