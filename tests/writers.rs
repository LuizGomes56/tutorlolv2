use rustc_hash::FxHashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tutorlolv2::writers::{self, CdnChampion};

include!(concat!(env!("OUT_DIR"), "/writers_test.rs"));

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let fetch_remote_url: Option<String> = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--fetch-remote=").map(|s| s.to_string()));

    let mut fx_map = Arc::<FxHashMap<String, CdnChampion>>::new(FxHashMap::default());

    let maybe_extern = if let Some(fetch_remote_url) = fetch_remote_url {
        let data: FxHashMap<String, CdnChampion> = reqwest::get(fetch_remote_url)
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let res_data = data
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v))
            .collect();
        fx_map = Arc::new(res_data);
        true
    } else {
        false
    };
    const MAX_CONCURRENT: usize = 16;
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let mut handles = Vec::new();

    let total_tests = writer_tests().len();
    println!(
        "🚀 Starting {} writer tests with {} concurrent workers...",
        total_tests, MAX_CONCURRENT
    );

    for (name, func) in writer_tests() {
        let semaphore = semaphore.clone();
        let fx_map = fx_map.clone();
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let data: Option<CdnChampion> = if maybe_extern {
                fx_map.get(name).map(|v| unsafe { std::ptr::read(v) })
            } else {
                None
            };
            let result: Result<_, _> = tokio::task::spawn_blocking(move || func(data)).await;
            (name, result)
        }));
    }

    let mut results = Vec::new();
    let mut failures = Vec::new();
    let mut completed = 0;

    for handle in handles {
        let (name, result) = handle.await.unwrap();
        completed += 1;

        match result.unwrap() {
            Ok(_) => {
                results.push(format!("✓ {}: PASSED", name));
                if completed % 10 == 0 || completed == total_tests {
                    println!("Progress: {}/{} tests completed", completed, total_tests);
                }
            }
            Err(e) => {
                let error_msg = format!("✗ {}: FAILED - {}", name, e);
                println!("❌ {}: FAILED", name);
                results.push(error_msg.clone());
                failures.push((name, e.to_string()));
            }
        }
    }

    let summary = format!(
        "Test Results Summary\n{}\n\nTotal tests: {}\nPassed: {}\nFailed: {}\n\n{}",
        "=".repeat(50),
        results.len(),
        results.len() - failures.len(),
        failures.len(),
        results.join("\n")
    );

    std::fs::write("test_results.txt", summary).expect("Failed to write test results");

    println!("\nFinal Results:");
    println!(
        "Total: {} | Passed: {} | Failed: {}",
        results.len(),
        results.len() - failures.len(),
        failures.len()
    );

    if !failures.is_empty() {
        println!("\n❌ Failed tests:");
        for (name, error_v) in &failures {
            println!("  - {} : {}", name, error_v);
        }

        panic!(
            "\n{} test(s) failed out of {}. Check test_results.txt for details.",
            failures.len(),
            results.len()
        );
    } else {
        println!("✅ All tests passed! Results saved to test_results.txt");
    }
}
