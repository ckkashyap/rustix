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

use core::str::StrExt;
use task::Cpu;
use x86asm::{readeflags, cli};
use mmu::FL_IF;
use console::panic;

pub struct Spinlock {
	locked: u32,
	name: &'static str,
    cpu: *mut Cpu
}

pub const DUMMY_LOCK: Spinlock = Spinlock{locked: 0, name: "", cpu: 0 as (*mut Cpu)};

pub fn init_lock(lk: &mut Spinlock, name : &'static str) {
	lk.name = name;
	lk.locked = 0;
    lk.cpu = 0 as (*mut Cpu);
}

impl Spinlock {
    pub fn holding(&self) -> bool {
        self.locked == 1u32 // TODO: cpu
    }
}

fn pushcli() {
    let eflags = readeflags();
    cli();
}

fn popcli() {
    // if (readeflags() & FL_IF) {
    //     panic("popcli - interruptible");
    // }
    // if(--cpu->ncli < 0)
    //     panic("popcli");
    // if(cpu->ncli == 0 && cpu->intena)
    //     sti();
}
