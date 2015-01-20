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
#![feature(lang_items, asm, intrinsics)]
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
mod mmu;
mod memlayout;
mod rlibc;
mod console;


fn main (end : u64) {
	unsafe {
		*((0xb8000 ) as *mut u8) = 65;
		*((0xb8001 ) as *mut u8) = 0x6;
		asm!("mov $$0xff, %eax" : /* no outputs */ : /* no inputs */ : "eax");
	}
	uart::early_init();
	kalloc::kinit1(end, memlayout::P2V(4*1024*1024));
}

#[no_mangle]
pub extern "C" fn cmain(end : u64)  {
	loop {
	main(end);
	}
        //return 255;
}



// Dummy functions to take care of missing libc function
#[no_mangle]
pub extern "C" fn trunc()  {console::panic("trunc: console::panic!!!");}
#[no_mangle]
pub extern "C" fn truncf()  {console::panic("truncf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn floor()  {console::panic("floor: console::panic!!!");}
#[no_mangle]
pub extern "C" fn floorf()  {console::panic("floorf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn pow()  {console::panic("pow: console::panic!!!");}
#[no_mangle]
pub extern "C" fn powf()  {console::panic("powf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn fmod()  {console::panic("fmod: console::panic!!!");}
#[no_mangle]
pub extern "C" fn fmodf()  {console::panic("fmodf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn log10()  {console::panic("log10: console::panic!!!");}
#[no_mangle]
pub extern "C" fn log10f()  {console::panic("log10f: console::panic!!!");}
#[no_mangle]
pub extern "C" fn memcpy()  {console::panic("memcpy: console::panic!!!");}
#[no_mangle]
pub extern "C" fn memcmp()  {console::panic("memcmp: console::panic!!!");}
#[no_mangle]
pub extern "C" fn log()  {console::panic("log: console::panic!!!");}
#[no_mangle]
pub extern "C" fn logf()  {console::panic("logf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn log2()  {console::panic("log2: console::panic!!!");}
#[no_mangle]
pub extern "C" fn log2f()  {console::panic("log2f: console::panic!!!");}
#[no_mangle]
pub extern "C" fn round()  {console::panic("round: console::panic!!!");}
#[no_mangle]
pub extern "C" fn roundf()  {console::panic("roundf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn exp()  {console::panic("exp: console::panic!!!");}
#[no_mangle]
pub extern "C" fn expf()  {console::panic("expf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn exp2()  {console::panic("exp2: console::panic!!!");}
#[no_mangle]
pub extern "C" fn exp2f()  {console::panic("exp2f: console::panic!!!");}
#[no_mangle]
pub extern "C" fn ceil()  {console::panic("ceil: console::panic!!!");}
#[no_mangle]
pub extern "C" fn ceilf()  {console::panic("ceilf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn fma()  {console::panic("fma: console::panic!!!");}
#[no_mangle]
pub extern "C" fn fmaf()  {console::panic("fmaf: console::panic!!!");}
#[no_mangle]
pub extern "C" fn __powisf2()  {console::panic("__powisf2: console::panic!!!");}
#[no_mangle]
pub extern "C" fn __powidf2()  {console::panic("__powidf2: console::panic!!!");}



#[lang = "stack_exhausted"] extern fn stack_exhausted() { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {loop {} }
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
