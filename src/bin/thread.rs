use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];

    thread::sleep(Duration::from_secs(2)); // 2秒スレッド作成を待つ

    for _ in 0..5 {
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_secs(5)); // スレッドの持続時間
        });

        handles.push(handle);
        thread::sleep(Duration::from_secs(1)); // 1秒ごとにスレッドを生成
    }

    for handle in handles {
        handle.join().unwrap();
    }
    thread::sleep(Duration::from_secs(1)); // 1秒メインスレッド終了を待つ
}
