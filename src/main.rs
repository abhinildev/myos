#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]
#![no_std]
#![no_main]
use core::panic::PanicInfo;
//use myos::println;
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
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    myos::test_panic_handler(info)
}
//static HELLO: &[u8]=b"Hello OS from os!!";
#[unsafe(no_mangle)]
pub extern "C" fn _start()-> !{
//    use core::fmt::Write;
//    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
//    write!(vga_buffer::WRITER.lock(),", some numbers: {} {}",42,1.337).unwrap();
    myos::init();
    x86_64::instructions::interrupts::int3();

   println!("Hello World{}","!");
   //panic!("Some panic message");
   #[cfg(test)]
   test_main();

   println!("It did not crash");
   loop{}
}
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