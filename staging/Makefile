# http://wiki.osdev.org/64-bit_Higher_Half_Kernel_with_GRUB_2

include $(SOURCE_ROOT)/Rules.inc

ISO := os.iso

OBJECTS=$(SOURCE_ROOT)/boot/x86_64/boot.o \
	$(SOURCE_ROOT)/kernel/libkernel.a
 
all:
	$(LD) -m elf_x86_64 -nodefaultlibs -T $(SOURCE_ROOT)/boot/x86_64/kernel64.ld -o kernel.sys $(SOURCE_ROOT)/boot/x86_64/entry64.o $(SOURCE_ROOT)/boot/x86_64/main.o $(SOURCE_ROOT)/kernel/libkernel.a -b binary $(SOURCE_ROOT)/boot/x86_64/initcode $(SOURCE_ROOT)/boot/x86_64/entryother
#$(LD) -nodefaultlibs -z max-page-size=0x1000 -Tlinker.ld -o kernel.sys --start-group $(OBJECTS) --end-group 


iso:
	cp kernel.sys iso/boot
	grub-mkrescue -o $(ISO) iso
	

clean:
	@rm -f $(ISO) iso/boot/kernel.sys kernel.sys
