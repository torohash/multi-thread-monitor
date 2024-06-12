
標準ライブラリのスレッド確認
```
cargo run --bin thread & PID=$! && \
cargo run --bin monitor $PID
```

tokioのスレッド確認
```
cargo run --bin tokio_thread & PID=$! && \
cargo run --bin tokio_monitor $PID
```

tokioランタイムがどのスレッドにタスクを割り振っているのか確認
```
cargo run --bin tokio_worker_monitor
```