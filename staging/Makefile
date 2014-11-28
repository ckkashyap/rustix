# http://wiki.osdev.org/64-bit_Higher_Half_Kernel_with_GRUB_2

include $(SOURCE_ROOT)/Rules.inc

ISO := os.iso

OBJECTS=$(SOURCE_ROOT)/boot/x86_64/boot.o \
	$(SOURCE_ROOT)/kernel/cmain.o \
	$(SOURCE_ROOT)/kernel/kernel.o
 
all:
	$(LD) -nodefaultlibs -Tlinker.ld -o kernel.sys --start-group $(OBJECTS) --end-group 


iso:
	cp kernel.sys iso/boot
	grub-mkrescue -o $(ISO) iso
	

clean:
	@rm -f $(ISO) iso/boot/kernel.sys kernel.sys
