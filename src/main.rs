#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod file_system;
use myos::println;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use myos::allocator;
    use myos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    use myos::task::{Task, executor::Executor};
    use myos::task::keyboard::print_keypress;
    println!("Hello World{}", "!");
    myos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    myos::task::scheduler::init();

    println!("Shell is ready. Type 'help' for commands.");
    println!("Try typing: run tasks");

    #[cfg(test)]
    {
        test_main();
        myos::hlt_loop();
    }

    #[cfg(not(test))]
    {
        let mut executor = Executor::new();

        executor.spawn(Task::new(shell_task()));

        executor.run()
    }
}

async fn shell_task() {
    use myos::shell;
    use myos::task::keyboard::ScancodeStream;
    use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1, DecodedKey};

    use futures_util::stream::StreamExt;

    let mut scancodes = ScancodeStream::new();

    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    myos::print!("myos> ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                shell::handle_key(key);
            }
        }
    }
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
