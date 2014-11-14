#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]

#[lang="sized"]
#[no_mangle]
pub extern "C" fn rust_func() -> int {
        return 255;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
