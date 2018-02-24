#![no_std]
#![feature(alloc, core_intrinsics, global_allocator, lang_items)]
#[macro_use] extern crate alloc;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_args: ::core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[no_mangle]
pub extern "C" fn run() {
    let mut vec = vec![1 as u8];
    vec.push(2);
}
