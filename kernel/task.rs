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
use core::marker::Copy;
use super::spinlock::{Spinlock, DUMMY_LOCK, init_lock};

pub const TASK_NUM: usize = 1024;

pub struct Task {
    sz: usize,
    pid: usize,
    killed: isize,
    name: &'static str
}

impl Copy for Task {}

struct TaskTable {
    lock: Spinlock,
    procs: &'static [Task; TASK_NUM]
}

static mut procs : TaskTable = TaskTable{
    lock: DUMMY_LOCK,
    procs: &[
        Task {
            sz: 0us,
            pid: 0us,
            killed: 0is,
            name: "undef"
        }; TASK_NUM]
};

pub fn init_proc() {
    unsafe {
        init_lock(&mut procs.lock, "task_table");
    }
}
