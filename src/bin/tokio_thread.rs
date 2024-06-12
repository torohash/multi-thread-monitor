use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    sleep(Duration::from_secs(2)).await; // 2秒スレッド作成を待つ
    
    for _ in 0..5 {
        let handle = task::spawn(async {
            sleep(Duration::from_secs(5)).await; // スレッドの持続時間
        });

        handles.push(handle);
        sleep(Duration::from_secs(1)).await; // 1秒ごとにスレッドを生成
    }

    for handle in handles {
        handle.await.unwrap();
    }
    sleep(Duration::from_secs(2)).await; // 2秒メインスレッド終了を待つ
}
