#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main="test_main"]

extern crate alloc;

use bootloader::{bootinfo, entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);
fn main(bootinfo:&'static BootInfo)->!{
   use myos::allocator;
    use myos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    myos::init();
    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&bootinfo.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}

}
#[panic_handler]
fn panic(info:&PanicInfo)->!{
    myos::test_panic_handler(info)
}
use alloc::vec::Vec;

#[test_case]
fn large_vec(){
    let n=1000;
    let mut vec=Vec::new();
    for i in 0..n{
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(),(n-1)*n/2);
}
use myos::allocator::HEAP_SIZE;
use alloc::{boxed::Box};
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}