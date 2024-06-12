#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(chron_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use chron_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");


    chron_os::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");

    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    chron_os::test_panic_handler(info)
}