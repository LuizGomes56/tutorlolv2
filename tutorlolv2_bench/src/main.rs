use bytes::Bytes;
use reqwest::Client;
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering::Relaxed},
    },
    time::Duration,
};
use tokio::{sync::Semaphore, time::Instant};

const URL: &str = "http://localhost:8082/api/games/v2/calculator";
const REQUESTS: usize = 1 << 19;
const CONCURRENCY: usize = 1 << 10;
const WARMUP_REQUESTS: usize = 256;
const RUNS: usize = 3;

static DATA: &[u8] = include_bytes!("testdata.bin");

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        .pool_max_idle_per_host(CONCURRENCY)
        .pool_idle_timeout(Duration::from_secs(30))
        .tcp_nodelay(true)
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    if WARMUP_REQUESTS > 0 {
        warmup(&client, WARMUP_REQUESTS).await?;
    }

    for run_idx in 1..=RUNS {
        println!("\n========== RUN {} / {} ==========", run_idx, RUNS);
        run_once(run_idx, &client).await?;
    }

    Ok(())
}

async fn run_once(run_idx: usize, client: &Client) -> anyhow::Result<()> {
    let body = Bytes::from_static(DATA);
    let sem = Arc::new(Semaphore::new(CONCURRENCY));
    let counter = Arc::new(AtomicU64::new(0));

    let mut lat_us = Vec::with_capacity(REQUESTS);
    let mut ok = 0usize;
    let mut err = 0usize;
    let mut non2xx = 0usize;

    let start_all = Instant::now();

    let mut joinset = tokio::task::JoinSet::new();
    for _ in 0..REQUESTS {
        let permit = sem.clone().acquire_owned().await?;
        let client = client.clone();
        let body = body.clone();
        let counter = counter.clone();

        joinset.spawn(async move {
            let _permit = permit;
            let req_id = counter.fetch_add(1, Relaxed) + 1;
            let t0 = Instant::now();

            let res = client
                .post(URL)
                .header("Content-Type", "application/octet-stream")
                .body(body)
                .send()
                .await;

            if let Err(ref e) = res {
                eprintln!("[run {run_idx} | req {req_id}] Error: {:#?}", e);
            }

            let elapsed = t0.elapsed();
            (req_id, res, elapsed)
        });
    }

    while let Some(join_res) = joinset.join_next().await {
        match join_res {
            Ok((_req_id, res, elapsed)) => {
                let micros = elapsed.as_micros() as u64;
                match res {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            ok += 1;
                            lat_us.push(micros);
                        } else {
                            non2xx += 1;
                        }
                    }
                    Err(_) => {
                        err += 1;
                    }
                }
            }
            Err(_) => {
                err += 1;
            }
        }
    }

    let total_elapsed = start_all.elapsed();

    lat_us.sort_unstable();
    let count = lat_us.len() as f64;
    let avg = if count > 0.0 {
        (lat_us.iter().sum::<u64>() as f64) / count
    } else {
        0.0
    };
    let p = |q: f64| -> u64 {
        if lat_us.is_empty() {
            return 0;
        }
        let idx = ((q * (lat_us.len() as f64 - 1.0)).round() as usize).min(lat_us.len() - 1);
        lat_us[idx]
    };

    println!("=== HTTP benchmark (run {run_idx}) ===");
    println!("URL           : {}", URL);
    println!("Requests      : {}", REQUESTS);
    println!("Concurrency   : {}", CONCURRENCY);
    println!("Warmup        : {}", WARMUP_REQUESTS);
    println!("Total time    : {:.3}s", total_elapsed.as_secs_f64());
    println!(
        "Throughput    : {:.1} req/s",
        (REQUESTS as f64) / total_elapsed.as_secs_f64()
    );
    println!("Success (2xx) : {}", ok);
    println!("Non-2xx       : {}", non2xx);
    println!("Errors        : {}", err);

    if !lat_us.is_empty() {
        println!(
            "Latency (µs)  : avg {:.0} | p50 {} | p90 {} | p95 {} | p99 {} | max {}",
            avg.round(),
            p(0.50),
            p(0.90),
            p(0.95),
            p(0.99),
            *lat_us.last().unwrap()
        );
    }

    Ok(())
}

async fn warmup(client: &Client, n: usize) -> anyhow::Result<()> {
    let body = Bytes::from_static(DATA);
    let mut js = tokio::task::JoinSet::new();
    for _ in 0..n {
        let c = client.clone();
        let b = body.clone();
        js.spawn(async move {
            let _ = c
                .post(URL)
                .header("Content-Type", "application/octet-stream")
                .body(b)
                .send()
                .await;
        });
    }
    while js.join_next().await.is_some() {}
    Ok(())
}
