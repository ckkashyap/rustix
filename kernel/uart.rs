// The MIT License (MIT)
// 
// Copyright (c) 2014 Kashyap
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


//use super::x86asm;

pub fn earlyinit () { 

	super::x86asm::outb(0x3f8 + 2 , 0);
	super::x86asm::outb(0x3f8+3, 0x80);    // Unlock divisor
	super::x86asm::outb(0x3f8+0, 12);
	super::x86asm::outb(0x3f8+1, 0);
	super::x86asm::outb(0x3f8+3, 0x03);    // Lock divisor, 8 data bits.
	super::x86asm::outb(0x3f8+4, 0);
	super::x86asm::outb(0x3f8+1, 0x01);    // Enable receive interrupts.


	let r = super::x86asm::inb(0x3f8+5);

	if r == 0xff {
		unsafe {
			*((0xb8002 ) as *mut u8) = 66;
			*((0xb8003 ) as *mut u8) = 0x4f;
			asm!("mov $$0xff, %eax" : /* no outputs */ : /* no inputs */ : "eax");
		}
	} else {
		unsafe {
			*((0xb8002 ) as *mut u8) = 65;
			*((0xb8003 ) as *mut u8) = 0x1f;
			asm!("mov $$0xff, %eax" : /* no outputs */ : /* no inputs */ : "eax");
		}
	}


}
