# The MIT License (MIT)
# 
# Copyright (c) 2014 Kashyap
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

INCLUDE=-I. -I $(SOURCE_ROOT)/include
CC=gcc

AT=@

RUST_SOURCES=$(wildcard *.rs)
C_SOURCES=$(wildcard *.c) 
ASM_SOURCES=$(wildcard *.s)
SOURCES=$(C_SOURCES) $(ASM_SOURCES)
C_OBJECTS=$(patsubst %.c,%.o,$(C_SOURCES))
RUST_OBJECTS=$(patsubst %.rs,%.o,$(RUST_SOURCES)
ASM_OBJECTS=$(patsubst %.s,%.o,$(ASM_SOURCES))
OBJECTS=$(C_OBJECTS) $(ASM_OBJECTS) 

CFLAGS= -O0 -static -Wall -nostartfiles -nostdlib       \
	-nodefaultlibs -nostdinc -fno-builtin           \
	-DKERNEL_VERSION="\"$(KERNEL_VERSION)\""        \
	-D_KERNEL_ -I$(INCLUDE)

%o:%rs
	@echo "$<"
	$(AT)rustc  $<  --target $(SOURCE_ROOT)/x86_64-avery-kernel.json  -Z no-landing-pads  --emit=obj
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

