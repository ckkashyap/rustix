INCLUDE=-I. -I $(SOURCE_ROOT)/include
CC=gcc

AT=@


C_SOURCES=$(wildcard *.c)
ASM_SOURCES=$(wildcard *.s)
SOURCES=$(C_SOURCES) $(ASM_SOURCES)
C_OBJECTS=$(patsubst %.c,%.o,$(C_SOURCES))
ASM_OBJECTS=$(patsubst %.s,%.o,$(ASM_SOURCES))
OBJECTS=$(C_OBJECTS) $(ASM_OBJECTS)

CFLAGS= -O0 -static -Wall -nostartfiles -nostdlib       \
	-nodefaultlibs -nostdinc -fno-builtin           \
	-DKERNEL_VERSION="\"$(KERNEL_VERSION)\""        \
	-D_KERNEL_ -I$(INCLUDE)

%o:%c
	@echo "$<"
	$(AT)$(CC) -c $(CFLAGS) $< -o $@
%o:%s
	@echo "$<"
	$(AT)nasm -felf64 $< -o $@

dummy: all
	
process_sub_dirs:
	$(AT)for file in $(SUB_DIRS); do make SOURCE_ROOT=$(SOURCE_ROOT) -C $$file; done

clean_sub_dirs:
	$(AT)for file in $(SUB_DIRS); do make SOURCE_ROOT=$(SOURCE_ROOT) -C $$file clean; done

library: $(OBJECTS)
	$(AT)ar -rc $(SOURCE_ROOT)/kernel/`basename $$PWD`.a $(OBJECTS)

LIBRARIES=$(patsubst %,%.a,$(SUB_DIRS))

