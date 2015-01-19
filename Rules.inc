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

LD=ld
AR=ar
QEMU=qemu-system-x86_64 -kernel staging/kernel.sys -serial stdio -vnc :1 -no-reboot -d int -s
LINUX_TARGET=x86_64-unknown-linux-gnu
MAC_TARGET=x86_64-apple-darwin
LINUX_RUST_DIR=$(RUST_ROOT)/rust-nightly-$(LINUX_TARGET)
MAC_RUST_DIR=$(RUST_ROOT)/rust-nightly-$(MAC_TARGET)
RUST_DIR=$(LINUX_RUST_DIR)
OBJDUMP=objdump
RUSTC=LD_LIBRARY_PATH=$(LINUX_RUST_DIR)/lib $(LINUX_RUST_DIR)/bin/rustc


OS=$(shell uname)
ifeq ($(OS), Darwin)
LD=$(LINKEDITOR)
AR=$(ARCHIVE)
LD_LIBRARY_PATH=DYLD_LIBRARY_PATH
QEMU=qemu-system-x86_64 -kernel staging/kernel.sys -serial stdio -no-reboot -d int
RUST_DIR=$(MAC_RUST_DIR)
RUSTC=DYLD_LIBRARY_PATH=$(MAC_RUST_DIR)/lib $(MAC_RUST_DIR)/bin/rustc
endif

AT=@

dummy: all

%o:%s
	@echo "$<"
	$(AT)nasm -felf64 $< -o $@

process_sub_dirs:
	$(AT)for file in $(SUB_DIRS); do make SOURCE_ROOT=$(SOURCE_ROOT) -C $$file; done

clean_sub_dirs:
	$(AT)for file in $(SUB_DIRS); do make SOURCE_ROOT=$(SOURCE_ROOT) -C $$file clean; done
