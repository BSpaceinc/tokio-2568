## `tokio::sync::Mutex`

```
cargo run --no-default-features --features mutex-tokio --release
    Finished release [optimized] target(s) in 0.08s
     Running `target\release\tokio-2568.exe`
tokio::sync::Mutex = 68ms
```

## `futures::lock::Mutex`

```
cargo run --no-default-features --features mutex-futures --release
    Finished release [optimized] target(s) in 0.09s
     Running `target\release\tokio-2568.exe`
futures::lock::Mutex = 57ms
```

## `async_std::sync::Mutex`

```
cargo run --no-default-features --features mutex-async-std --release
    Finished release [optimized] target(s) in 0.09s
     Running `target\release\tokio-2568.exe`
async_std::sync::Mutex = 58ms
```

## `futures-intrusive(fair = false)`

```
cargo run --no-default-features --features mutex-futures-intrusive --release
    Finished release [optimized] target(s) in 0.08s
     Running `target\release\tokio-2568.exe`
futures_intrusive::sync::Mutex(fair = false) = 63ms
```

## `futures-intrusive(fair = true)`

```
cargo run --no-default-features --features mutex-futures-intrusive-fair --release
    Finished release [optimized] target(s) in 0.09s
     Running `target\release\tokio-2568.exe`
futures_intrusive::sync::Mutex(fair = true) = 62ms
```
