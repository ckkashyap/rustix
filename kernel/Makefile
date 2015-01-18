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

include $(SOURCE_ROOT)/Rules.inc

all: libcompiler-rt.a libmorestack.a
	cp -f $(LINUX_RUST_DIR)/lib/rustlib/$(LINUX_TARGET)/lib/libcore-*.rlib libcore.rlib
	$(RUSTC) --target=$(LINUX_TARGET) -C no-stack-check kernel.rs  --extern core=libcore.rlib -L.

libcore.rlib:
	cp $(LINUX_RUST_DIR)/lib/rustlib/$(LINUX_TARGET)/lib/libcore-*.rlib libcore.rlib
	#$(RUSTC) --target=$(TARGET) -C no-stack-check --crate-type lib $(LIB_RS) --crate-name core

libcompiler-rt.a:
	cp $(LINUX_RUST_DIR)/lib/rustlib/$(LINUX_TARGET)/lib/libcompiler-rt.a .

libmorestack.a:
	cp $(LINUX_RUST_DIR)/lib/rustlib/$(LINUX_TARGET)/lib/libmorestack.a .

clean:
	rm -f libkernel.a
