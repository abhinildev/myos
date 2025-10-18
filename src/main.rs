#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]
#![no_std]
#![no_main]
use core::panic::PanicInfo;
use bootloader::{BootInfo,entry_point};
use myos::{allocator, memory::{self, translate_addr, EmptyFrameAllocator}};
use x86_64::{addr, structures::paging::{Page, PageTable}};
use alloc::{boxed::Box,vec,vec::Vec,rc::Rc};

//use myos::println;
extern crate alloc;
mod vga_buffer;
mod serial;
#[test_case]
fn trivial_assertion(){
    // print!("trivial assertion ....");
    // assert_eq!(1,1);
    // println!("[ok]");
    //serial_print!("trivial assertion... ");
    assert_eq!(1,1);
    //serial_println!("[ok]");
}
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]){
    //println!("Running {} tests",tests.len());
    serial_print!("Running {} tests",tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(QemuExitCode::Sucess);
}
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo)-> ! {
    println!("{}", info);
    myos::hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    myos::test_panic_handler(info)
}
entry_point!(kernal_main);
fn kernal_main(boot_info: &'static BootInfo)->!{
    use myos::memory::BootInfoFrameAllocator;
    //use x86_64::structures::paging::Translate;
    use myos::allocator;
    //use myos::memory::active_level_4_table;
    use x86_64::VirtAddr;
    println!("Hellp World{}","!");
    myos::init();
   let phys_mem_offset=VirtAddr::new(boot_info.physical_memory_offset);
   let mut mapper=unsafe{memory::init(phys_mem_offset)};
   let mut frame_allocator=unsafe{
    BootInfoFrameAllocator::init(&boot_info.memory_map)
   };
   allocator::init_heap(&mut mapper,&mut  frame_allocator)
   .expect("Heap allocation failed");
   let heap_value=Box::new(41);
   println!("Heap value at {:p}",heap_value);

   let mut vec=Vec::new();
   for i  in 0..500{
    vec.push(i);
   }
   println!("vec at {:p} ",vec.as_slice());
   let reference_counted=Rc::new(vec![1,2,3]);
   let cloned_referenc=reference_counted.clone();
   println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_referenc)
   );
   core::mem::drop(reference_counted);
   println!(
    "reference count is {} now",
    Rc::strong_count(&cloned_referenc)
   );

//    let page=Page::containing_address(VirtAddr::new(0));
//    memory::create_mapping(page,&mut mapper,&mut frame_allocator);
//    let page_ptr:*mut u64 =page.start_address().as_mut_ptr();
//    unsafe {page_ptr.offset(400).write_volatile(0x_f077_f065_f04e);}

//    let addresses=[
//     0xb8000,
//     0x201008,
//     0x100_0020_1a10,
//    ];
//    for &address in &addresses{
//     let virt=VirtAddr::new(address);
//     let phys=mapper.translate_addr(virt);
//     println!("{:?} ->{:?}",virt,phys);
//    }
   

    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    myos::hlt_loop();
}
//static HELLO: &[u8]=b"Hello OS from os!!";
// #[unsafe(no_mangle)]
// pub extern "C" fn _start(boot_info: &'static BootInfo)-> !{
// //    use core::fmt::Write;
// //    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
// //    write!(vga_buffer::WRITER.lock(),", some numbers: {} {}",42,1.337).unwrap();
//     myos::init();
//     // unsafe {
//     //     *(0xdeadbeef as *mut u8)=42;
//     // };
//     //x86_64::instructions::interrupts::int3();
//     use x86_64::registers::control::Cr3;

//     let (level_4_page_table,_)=Cr3::read();
//     println!("Level 4 page table at: {:?}",level_4_page_table.start_address());
//     //stack_overflow();
//     let ptr=0xdeadbeef as *mut u8;
//     unsafe {*ptr=42;}

//    println!("Hello World{}","!");
//    //panic!("Some panic message");
//    #[cfg(test)]
//    test_main();

//    println!("It did not crash");
//    myos::hlt_loop();
// }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Sucess=0x10,
    Failed=0x11,
}
pub fn exit_qemu(exit_code:QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port =Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
pub trait Testable {
    fn run(&self)->();
}
impl <T> Testable for T
where 
    T:Fn(),
{
    fn run(&self){
        serial_print!("{}...\t",core::any::type_name::<T>());
        self();
        serial_print!("[ok]");
    }
    
}