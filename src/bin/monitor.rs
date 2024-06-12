use std::process::Command;
use std::io::{self, Write};
use std::env;

fn get_tids(pid: u32) -> Vec<u32> {
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

fn main() {
    let pid = env::args()
        .nth(1)
        .expect("Please provide a PID")
        .parse::<u32>()
        .expect("Invalid PID");

    loop {
        let tids = get_tids(pid);
        print!("\x1B[2J\x1B[1;1H"); // 画面をクリア
        println!("PID: {}", pid);
        println!("TIDs: {:?}", tids);

        io::stdout().flush().unwrap();
        // 1秒待機
        std::thread::sleep(std::time::Duration::from_secs(1));

        if tids.len() == 0 {
            break;
        }
    }
}
