use libc;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let pid = unsafe { libc::getpid() };
    println!("PID: {}", pid);

    let mut handles = vec![];

    for _ in 0..5 {
        let handle = task::spawn(async move {
            let tid = unsafe { libc::syscall(libc::SYS_gettid) };
            println!("TID: {:?}", tid);
            sleep(Duration::from_secs(5)).await; // スレッドの持続時間
        });

        handles.push(handle);
        sleep(Duration::from_secs(1)).await; // 1秒ごとにスレッドを生成
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
