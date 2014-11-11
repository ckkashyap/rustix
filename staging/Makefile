all:
	cp $$SOURCE_ROOT/kernel/kernel iso
	cp menu.lst iso/boot/grub
	mkisofs -o $$SOURCE_ROOT/boot.iso -b boot/grub/stage2_eltorito  -no-emul-boot -c boot.cat  -boot-load-size 32 -boot-info-table  -l -L -J -R -r iso

clean:
	rm -f iso/kernel
	rm -f $$SOURCE_ROOT/boot.iso
