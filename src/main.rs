#![no_std]
#![no_main]
// Setup custom testing
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use blog_os::{print, println};

// Entry point for the program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("'ullo governer");

    blog_os::init();

    #[cfg(test)]
    test_main();

    println!("Blimey! It didn't crash!");
    // loop {
    //     for _ in 0..10000 {
    //         volatile::Volatile::new(0).read();
    //     }
    //     print!("-");
    // }
    blog_os::hlt_loop();
}

// Called on a panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
