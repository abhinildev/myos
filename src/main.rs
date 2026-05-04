#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod file_system;
use myos::println;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

use myos::shell;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use myos::allocator;
    use myos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    println!("Hello World{}", "!");
    myos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    println!("Shell is ready. Type 'help' for commands.");

    #[cfg(test)]
    {
    test_main();
    myos::hlt_loop();
    }

    #[cfg(not(test))]
    shell::run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    myos::hlt_loop();
}

async fn example_task() {
    println!("[task] example_task: starting");
    println!("[task] example_task: multitasking is working");
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
