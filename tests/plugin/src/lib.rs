//! # Plugin test
//! 

#![no_std]
#![feature(lang_items)]

extern crate ruspiro_allocator;
extern crate alloc;
use alloc::{boxed::Box, sync::Arc};
use ruspiro_singleton::Singleton;
use ruspiro_gpio_hal::*;

#[no_mangle]
pub extern "C" fn plugin_entry(gpio: Arc<Singleton<Box<dyn Gpio>>>) -> ! {
    loop {}
}



use core::panic::PanicInfo;

#[panic_handler]
#[allow(clippy::empty_loop)]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {
    // for the time beeing - nothing to be done as the usecase is a bit unclear
}