#!/bin/bash

```
cargo run --bin thread & PID=$! && \
cargo run --bin monitor $PID
```

```
cargo run --bin tokio_thread & PID=$! && \
cargo run --bin tokio_monitor $PID
```