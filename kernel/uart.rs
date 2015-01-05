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

use super::x86asm::outb;
use super::x86asm::inb;
use super::picirq::pic_enable;

use core::str::StrExt;



const COM1 : u16 = 0x3f8;
static mut uart_initialized : bool = false;


pub fn early_init () { 
	outb(COM1+2 , 0);
	outb(COM1+3, 0x80);    // Unlock divisor
	outb(COM1+0, 12);
	outb(COM1+1, 0);
	outb(COM1+3, 0x03);    // Lock divisor, 8 data bits.
	outb(COM1+4, 0);
	outb(COM1+1, 0x01);    // Enable receive interrupts.


	if inb(COM1+5) == 0xff {
		return;
	}

	unsafe {
		uart_initialized = true;
	}

	uart_put_str("xv6 initizing...\n");
}



fn uartinit()
{
	unsafe {
		if !uart_initialized {
			return;
		}
	}

	// Acknowledge pre-existing interrupt conditions;
	// enable interrupts.
	inb(COM1+2);
	inb(COM1+0);
	pic_enable(0);
	//ioapicenable(IRQ_COM1, 0);
}


fn uart_put_str(text: &str) {
	for b in text.bytes() {
		outb(COM1, b);
	}
}




fn uart_putc(byte : u8) {
	outb(COM1, byte);
}
