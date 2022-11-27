/* main.rs */

//#![feature(asm)]
/* no longer requires an atribute to enabel */

#![no_std]  /* don't link the Rust standard library*/
#![no_main] /* disable all Rust-level entry points */

use core::panic::PanicInfo;


static HELLO: &[u8] = b"Good Morning Starshine, the Earth Says Hello!";


#[no_mangle]    /* used to give the name of the function, usally
                 * gives a cryptic symbol to give every function
                 * a unique name.*/
pub extern "C" fn _start() -> ! {
        /* extern "C" to tell the compiler that it should use the "C calling
         * convention for this function *instead of the unspecified Rust calling
         * convention). The name for _start, is for use as the default entry
         * point name for most system. The "!" return type means function is
         * diverging i.e. not allowed to ever return */
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe { /* Unsafe code */
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/* This function is called on panic */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


// vim: tw=80
// vim: colorcolumn=81
