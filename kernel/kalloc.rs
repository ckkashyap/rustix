// The MIT License (MIT)
// 
// Copyright (c) 2015 Kashyap
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

use super::spinlock::spinlock;
use super::spinlock::dummy_lock;
use super::spinlock::init_lock;
use super::uart::uart_put_str;

struct kmem_t{
lock: spinlock,
use_lock: u32 , //TODO is u32 the right type?
	    // TODO  struct run *freelist;
}


static mut kmem : kmem_t = kmem_t { lock: dummy_lock, use_lock: 0} ;


pub fn kinit1(vstart: u64, vend: u64)
{
	unsafe {
		init_lock(& mut kmem.lock, "kmem");
		kmem.use_lock = 0;
	}
	//freerange(vstart, vend);
}
