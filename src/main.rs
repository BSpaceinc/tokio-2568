use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::time::Instant;

use async_trait::async_trait;

mod dataloader;
mod mutex;
use self::dataloader::{BatchFn, Loader};

struct MyLoadFn;

#[async_trait]
impl BatchFn<usize, usize> for MyLoadFn {
    async fn load(&self, keys: &[usize]) -> HashMap<usize, usize> {
        keys.iter()
            .map(|v| (v.clone(), v.clone()))
            .collect::<HashMap<_, _>>()
    }
}

#[tokio::main]
async fn main() {
    let n = 500;
    let t = Instant::now();
    bench_mutex(n).await;
    let d = Instant::now() - t;
    println!("{} = {}ms", mutex::NAME, d.as_millis());
}

async fn bench_mutex(n: usize) {
    let loader = Loader::new(MyLoadFn);

    for key in 0..n {
        let loader = loader.clone();
        let batch = FuturesUnordered::new();
        for _ in 0..n {
            let loader = loader.clone();
            batch.push(async move {
                loader.load(key).await;
            });
        }
        batch.collect::<Vec<_>>().await;
    }
}
