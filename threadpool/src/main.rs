use std::sync::mpsc::channel;
use cpus::get;
use threadpool::*;

mod cpus;
mod threadpool;

fn main() {
    println!("cpu count: {}", get());

    let n_workers = get();
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1).expect("sending data back from the threadpool");
        });
    }

    println!("result: {}", rx.iter().take(n_jobs).fold(0, |a, b| a + b));
}

