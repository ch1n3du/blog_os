#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
static SIC_MUNDUS: &[u8] = b"Sic Mundus Creatus Est";

// Entry point for the program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // Prints "Hello World!" to the VGA buffer.
    for (i, &byte) in SIC_MUNDUS.iter().enumerate() {
        unsafe {
            // Write character byte then the byte for the color black
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
