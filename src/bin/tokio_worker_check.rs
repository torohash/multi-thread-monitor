use libc;
use tokio::task;
use tokio::time::{sleep, Duration};
use std::os::raw::c_long;

#[tokio::main]
async fn main() {
    let main_tid = unsafe { libc::pthread_self() };
    let main_sys_tid = unsafe { libc::syscall(libc::SYS_gettid) as c_long };
    println!("Main thread ID: {:?}, System TID: {}", main_tid, main_sys_tid);

    let mut handles = vec![];

    for i in 0..5 {
        let handle = task::spawn(async move {
            let tid = unsafe { libc::pthread_self() };
            let sys_tid = unsafe { libc::syscall(libc::SYS_gettid) as c_long };
            println!("Worker thread ID: {:?}, System TID: {}", tid, sys_tid);
            sleep(Duration::from_secs(5)).await; // スレッドの持続時間
        });

        handles.push(handle);
        sleep(Duration::from_secs(1)).await; // 1秒ごとにスレッドを生成
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
