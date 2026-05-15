use crate::println;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::task::executor::Executor;
use crate::task::Task;
use core::sync::atomic::{AtomicU8, Ordering};


static SPAWN_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static TASKS_DONE: AtomicU8 = AtomicU8::new(0);

pub fn init() {
    SPAWN_QUEUE
    .try_init_once(|| ArrayQueue::new(10))
    .expect("scheduler::init should only be called once");
}

pub fn request_spawn()  {
    if let Ok(queue) = SPAWN_QUEUE.try_get() {
        queue.push(1).ok();
        println!("[scheduler] Task spawn requested");
    }
}

pub fn should_spawn() -> bool {
    if let Ok(queue) = SPAWN_QUEUE.try_get() {
        queue.pop().is_some()
    } else {
        false
    }
}

pub async fn task_one() {
    println!("[task 1) Starting: Initializing filesystem check...");
    YieldNow::new().await;
    println!("[task 1] Scanning directory entries...");
    YieldNow::new().await;
    println!("[task 1] Done: Filesystem looks healthy!");
    check_all_done();  
}

pub async fn task_two() {
    println!("[task 2] Starting: Checking memory regions...");
    YieldNow::new().await;
    println!("[task 2] Heap usage looks normal...");
    YieldNow::new().await;
    println!("[task 2] Done: Memory check passed!");
    check_all_done();  
}
 
pub async fn task_three() {
    println!("[task 3] Starting: Processing input buffer...");
    YieldNow::new().await;
    println!("[task 3] Parsing data chunks...");
    YieldNow::new().await;
    println!("[task 3] Done: Buffer processed successfully!");
    check_all_done();  
}
 
pub async fn task_four() {
    println!("[task 4] Starting: Writing output to VGA...");
    YieldNow::new().await;
    println!("[task 4] Flushing write buffer...");
    YieldNow::new().await;
    println!("[task 4] Done: Output written!");
    check_all_done();  
}
 
pub async fn task_five() {
    println!("[task 5] Starting: Running cleanup routines...");
    YieldNow::new().await;
    println!("[task 5] Releasing temporary resources...");
    YieldNow::new().await;
    println!("[task 5] Done: Cleanup complete!");
    check_all_done();  
}

fn check_all_done() {
    let done = TASKS_DONE.fetch_add(1, Ordering::Relaxed) + 1;
    if done >= 5 {
        TASKS_DONE.store(0, Ordering::Relaxed);
        println!("----------------------------------");
        println!("[scheduler] All tasks completed!");
        println!("----------------------------------");
        crate::print!("myos> ");
    }
}

pub fn spawn_all_tasks(executor: &mut Executor) {
    println!("[scheduler] Spawning 5 tasks...");
    println!("----------------------------------");

    static COUNTER: core::sync::atomic::AtomicU64 
        = core::sync::atomic::AtomicU64::new(0);
    let order = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

    match order % 5 {
        0 => {
            executor.spawn(Task::new(task_one()));
            executor.spawn(Task::new(task_two()));
            executor.spawn(Task::new(task_three()));
            executor.spawn(Task::new(task_four()));
            executor.spawn(Task::new(task_five()));
        }
        1 => {
            executor.spawn(Task::new(task_three()));
            executor.spawn(Task::new(task_one()));
            executor.spawn(Task::new(task_five()));
            executor.spawn(Task::new(task_two()));
            executor.spawn(Task::new(task_four()));
        }
        2 => {
            executor.spawn(Task::new(task_five()));
            executor.spawn(Task::new(task_four()));
            executor.spawn(Task::new(task_three()));
            executor.spawn(Task::new(task_two()));
            executor.spawn(Task::new(task_one()));
        }
        3 => {
            executor.spawn(Task::new(task_two()));
            executor.spawn(Task::new(task_four()));
            executor.spawn(Task::new(task_one()));
            executor.spawn(Task::new(task_five()));
            executor.spawn(Task::new(task_three()));
        }
        _ => {
            executor.spawn(Task::new(task_four()));
            executor.spawn(Task::new(task_two()));
            executor.spawn(Task::new(task_five()));
            executor.spawn(Task::new(task_one()));
            executor.spawn(Task::new(task_three()));
        }
    }

    println!("[scheduler] All 5 tasks spawned!");
    println!("----------------------------------");
}

struct YieldNow {
    yielded: bool,
}

impl YieldNow {
    fn new() -> Self {
        YieldNow { yielded: false}
    }
}

impl core::future::Future for YieldNow {
    type Output = ();

    fn poll(

        mut self: core ::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<()> {
        if self.yielded {
            core::task::Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    }
}