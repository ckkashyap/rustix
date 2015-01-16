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



#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![feature(lang_items, asm)]
#![no_std]
#[lang="sized"]
#[lang="sync"]

extern crate core;

mod uart;
mod x86asm;
mod picirq;
mod traps;
mod spinlock;
mod kalloc;


fn kashyap () {
	unsafe {
		*((0xb8000 ) as *mut u8) = 65;
		*((0xb8001 ) as *mut u8) = 0x6;
		asm!("mov $$0xff, %eax" : /* no outputs */ : /* no inputs */ : "eax");
	}
	uart::early_init();
	kalloc::kinit1(0,0);
}

#[no_mangle]
pub extern "C" fn cmain()  {
	loop {
	kashyap();
	}
        //return 255;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {loop {} }
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
