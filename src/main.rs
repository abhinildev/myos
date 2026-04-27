#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
mod file_system;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use myos::println;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

use crate::file_system::fs::FileSystem;

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

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );
    let mut fs=FileSystem::new();
    fs.init();
    fs.create("a.txt");
    fs.write_file("a.txt", "hi!!");
    fs.read_file("a.txt");
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    //multitasking
    use myos::task::{Task, executor::Executor};
    use myos::task::keyboard::print_keypress;

    let mut executor = Executor::new();

    executor.spawn(Task::new(print_keypress()));

    executor.spawn(Task::new(example_task()));

    executor.run()
    
    //myos::hlt_loop();
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
