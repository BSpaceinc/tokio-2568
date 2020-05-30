## `tokio::sync::Mutex`

```
cargo run --no-default-features --features mutex-tokio
    Finished dev [unoptimized + debuginfo] target(s) in 1.55s
     Running `target\debug\tokio-2568.exe`
tokio::sync::Mutex = 1379ms
```

## `futures::lock::Mutex`

```
cargo run --no-default-features --features mutex-futures
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target\debug\tokio-2568.exe`
futures::lock::Mutex = 857ms
```

## `async_std::sync::Mutex`

```
cargo run --no-default-features --features mutex-async-std
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target\debug\tokio-2568.exe`
async_std::sync::Mutex = 882ms
```
