use std::process::Command;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};
use std::env;

async fn get_tids(pid: u32) -> Vec<u32> {
    let output = Command::new("ps")
        .args(&["-T", "-p", &pid.to_string()])
        .output()
        .expect("Failed to execute ps command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut tids = vec![];

    for line in output_str.lines().skip(1) {
        if let Some(tid_str) = line.split_whitespace().nth(1) {
            if let Ok(tid) = tid_str.parse::<u32>() {
                tids.push(tid);
            }
        }
    }

    tids
}

#[tokio::main]
async fn main() {
    let pid = env::args()
        .nth(1)
        .expect("Please provide a PID")
        .parse::<u32>()
        .expect("Invalid PID");

    loop {
        let tids = get_tids(pid).await;
        print!("\x1B[2J\x1B[1;1H"); // 画面をクリア
        println!("PID: {}", pid);
        println!("TIDs: {:?}", tids);

        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await; // 1秒ごとに更新

        if tids.len() == 0 {
            break;
        }
    }
}
