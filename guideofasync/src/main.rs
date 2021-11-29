use futures::future::join_all;
use futures::prelude::*;
use tokio::runtime::*;
use tokio::task;

use log::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn our_async_program() -> Result<String> {
    future::ok("Hello world".to_string()).await
}

fn fib_cpu_intensive(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n - 1) + fib_cpu_intensive(n - 2),
    }
}

fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/https://www.163.com/",
        delay_ms,
    );

    reqwest::Url::parse(&url).unwrap()
}

async fn request(n: usize) -> Result<()> {
    reqwest::get(slowwly(1000)).await?;

    info!("Got response {}", n);

    Ok(())
}


async fn get_and_analyze(n: usize) -> Result<(u64, u64)> {
    let response: reqwest::Response = reqwest::get(slowwly(1000)).await?;
    info!("Dataset {}", n);

    // we get the body of the request
    let txt = response.text().await?;

    // we send our analysis work to a thread where there is no runtime running
    // we don't block the runtime by analyzing the data
    let res = task::spawn_blocking(move || analyze(&txt)).await?;
    info!("Processed {}", n);

    Ok(res)
}

// Counting the number of ones and zeros in the bytes we get
fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();

    // Let's spend as much time as we can and count them in two passes
    let ones = txt.iter().fold(0u64, |acc, b| acc + b.count_ones() as u64);
    let zeros = txt.iter().fold(0u64, |acc, b| acc + b.count_zeros() as u64);

    (ones, zeros)
}

async fn app() -> Result<()> {
    let _concurrent_future = task::spawn(our_async_program());

    let threadpool_future = task::spawn_blocking(|| fib_cpu_intensive(20));

    let res = threadpool_future.await?;
    info!("The result of fib cpu intnesive: {}", res);

    info!("Start program!");

    let resp1 = task::spawn(request(1));
    let resp2 = task::spawn(request(2));

    let _ = resp1.await??;
    let _ = resp2.await??;

    // this is new. we can collect futures in a collection. nice to know!
    let mut futures = vec![];

    for i in 1..=10 {
        let fut = task::spawn(get_and_analyze(i));
        futures.push(fut);
    }

    let results = join_all(futures).await;

    let mut total_ones = 0;
    let mut total_zeros = 0;

    // Returning errors using `?` in iterators can be a bit difficult. Using a
    // simple for loop to inspect and work with our results can often be more
    // ergonomic
    for result in results {
        // `spawn_blocking` returns a `JoinResult` we need to unwrap first
        let ones_res: Result<(u64, u64)> = result?;
        let (ones, zeros) = ones_res?;

        total_ones += ones;
        total_zeros += zeros;
    }

    info!(
        "Ratio of ones/zeros: {:.02}",
        total_ones as f64 / total_zeros as f64
    );

    Ok(())
}

fn main() {
    use std::io::Write;
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();

    let rt = Runtime::new().unwrap();

    match rt.block_on(app()) {
        Ok(_) => info!("Done"),
        Err(e) => error!("An error occured: {}", e),
    }


    let mut y = Box::new(0);
    let z = y;

    y = Box::new(1);

    println!("{} {}", z, y);
}
