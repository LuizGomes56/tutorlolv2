use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{task::JoinSet, time::Instant};
use wiki::{JsonRead, read_to_string, stats::*};

#[tokio::test(start_paused = true)]
async fn queued_requests_wait_for_a_later_pause() {
    let limiter = Arc::new(Limiter::new());
    limiter.wait().await;

    let mut queued = JoinSet::new();
    for _ in 0..CONCURRENCY {
        let limiter = limiter.clone();
        queued.spawn(async move {
            limiter.wait().await;
            Instant::now()
        });
    }

    // Let every task reserve a slot before the 429 arrives.
    tokio::task::yield_now().await;
    let resume_at = Instant::now() + Duration::from_secs(1);
    limiter.pause(Duration::from_secs(1)).await;

    let mut sent = Vec::new();
    while let Some(result) = queued.join_next().await {
        sent.push(result.unwrap());
    }
    sent.sort();
    assert!(sent.iter().all(|at| *at >= resume_at));
    assert!(
        sent.windows(2)
            .all(|pair| pair[1] - pair[0] >= REQUEST_INTERVAL)
    );
}

#[tokio::test(start_paused = true)]
async fn queued_requests_observe_an_extended_pause() {
    let limiter = Arc::new(Limiter::new());
    limiter.wait().await;
    let queued = {
        let limiter = limiter.clone();
        tokio::spawn(async move {
            limiter.wait().await;
            Instant::now()
        })
    };
    tokio::task::yield_now().await;
    limiter.pause(Duration::from_millis(200)).await;
    tokio::time::advance(Duration::from_millis(100)).await;
    let resume_at = Instant::now() + Duration::from_millis(300);
    limiter.pause(Duration::from_millis(300)).await;

    assert!(queued.await.unwrap() >= resume_at);
}

#[test]
fn match_lists_are_refreshed_for_a_new_patch() {
    // Use a child test process so relative cache paths stay isolated without
    // changing the working directory of concurrently running tests.
    const CHILD: &str = "TUTORLOLV2_STATS_CACHE_TEST_CHILD";
    const VERSION: &str = "16.21.1";
    const PATCH: &str = "16.21";
    if std::env::var_os(CHILD).is_some() {
        let api = Arc::new(RiotApi {
            key: String::new(),
            limiters: HashMap::new(),
        });
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(api.download_platform("kr", "asia", 1))
            .unwrap();
        let ids = BTreeSet::<String>::from_file(patch_cache("kr").join("match_ids.json")).unwrap();
        assert!(
            ids.is_empty(),
            "reused matches from an older patch: {ids:?}"
        );
        assert_eq!(read_to_string(match_path("KR_1")).unwrap(), "cached match");
        return;
    }

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("tutorlolv2-stats-{}-{unique}", std::process::id()));
    for dir in [
        "cache/stats/kr".to_string(),
        "cache/stats/kr/0.0".into(),
        format!("cache/stats/kr/{PATCH}"),
    ] {
        std::fs::create_dir_all(root.join(&dir)).unwrap();
        std::fs::write(root.join(&dir).join("players.json"), "[]").unwrap();
    }
    for dir in ["cache/stats/kr", "cache/stats/kr/0.0"] {
        std::fs::write(root.join(dir).join("match_ids.json"), r#"["KR_1"]"#).unwrap();
    }
    std::fs::create_dir_all(root.join("cache/stats/matches")).unwrap();
    std::fs::write(root.join("cache/stats/matches/KR_1.json"), "cached match").unwrap();

    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "match_lists_are_refreshed_for_a_new_patch",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .env("LOL_VERSION", VERSION)
        .current_dir(&root)
        .output()
        .unwrap();
    std::fs::remove_dir_all(root).unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
