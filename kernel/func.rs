#![crate_name = "kernel"]
#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]
#[lang="sized"]

fn kashyap () {
}

#[no_mangle]
pub extern "C" fn main()  {
	loop {
		kashyap();
	}
        //return 255;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
