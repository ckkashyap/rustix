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

const IO_PIC1 : u16 = 0x20;// Master (IRQs 0-7)
const IO_PIC2 : u16 = 0xA0;// Slave (IRQs 8-15)

const IRQ_SLAVE : u16 = 2;// IRQ at which slave connects to master

static mut irq_mask : u16 = 0xFFFF & !( 1 << (IRQ_SLAVE as usize));

pub fn pic_setmask(mask : u16)
{
	unsafe {
		irq_mask = mask;
	}
	outb(IO_PIC1+1, mask as u8); // TODO - check out a better way to do it
	outb(IO_PIC2+1, (mask >> 8) as u8);
}



pub fn pic_enable(irq : u8)
{
	unsafe {
		pic_setmask(irq_mask & !(1 << irq as usize));
	}
}


