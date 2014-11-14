all:
	make SOURCE_ROOT=$$PWD -C boot/x86_64
	make SOURCE_ROOT=$$PWD -C kernel
	make SOURCE_ROOT=$$PWD -C staging
clean:
	make SOURCE_ROOT=$$PWD -C boot/x86_64 clean
	make SOURCE_ROOT=$$PWD -C kernel clean
	make SOURCE_ROOT=$$PWD -C staging clean
