#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(chron_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use chron_os::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use chron_os::task::simple_executor::SimpleExecutor;
use chron_os::task::{keyboard, Task};
use chron_os::task::executor::Executor;

extern crate alloc;
entry_point!(kernel_main);

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use chron_os::allocator;
    use chron_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    // uncomment lines below to trigger a stack overflow
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // stack_overflow();

    // Cause a page-fault exception "CAUSED_BY_WRITE"
    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe { *ptr = 42; }

    println!("Hello World{}", "!");
    chron_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // Above we initialized interrupts, mapper, frame_allocator, and heap
    #[cfg(test)]
    test_main();
    //////////////////////////////////////////

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    chron_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    chron_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}