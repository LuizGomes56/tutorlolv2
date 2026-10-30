use clap::Parser;
use serde_json::json;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use tutorlolv2_perf::{
    Result, Settings, fingerprint,
    realtime::{RealtimeWorkload, generate_realtime, run_realtime},
};

#[derive(Parser)]
#[command(about = "Generate/replay seeded games and measure realtime calls; use --profile perf")]
struct Args {
    /// Enemy count when generating a new fixture (zero is supported).
    #[arg(long, default_value_t = 5)]
    enemies: usize,
    /// Number of different games in the replay corpus.
    #[arg(long, default_value_t = 32)]
    games: usize,
    #[arg(long, default_value_t = 42)]
    seed: u64,
    /// Allies excluding the active player.
    #[arg(long, default_value_t = 4)]
    allies: usize,
    /// Existing JSON is reused; generation flags apply only to new fixtures.
    #[arg(long)]
    input: Option<PathBuf>,
    /// Replace the fixture with a newly generated corpus.
    #[arg(long)]
    regenerate: bool,
    #[arg(long)]
    generate_only: bool,
    /// Realtime calls in EACH measured sample.
    #[arg(long, default_value_t = 500)]
    iterations: usize,
    #[arg(long, default_value_t = 20)]
    samples: usize,
    #[arg(long, default_value_t = 200)]
    warmup: usize,
    /// Report JSON path; an existing result is never overwritten.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

fn command(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn execute(args: Args) -> Result<()> {
    let path = args.input.unwrap_or_else(|| {
        PathBuf::from(format!(
            "perf/fixtures/realtime-n{}-a{}-g{}-s{}.json",
            args.enemies, args.allies, args.games, args.seed
        ))
    });

    if args.regenerate || !path.exists() {
        let bytes = generate_realtime(args.seed, args.games, args.enemies, args.allies)?;

        parent(&path)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(args.regenerate)
            .create_new(!args.regenerate)
            .open(&path)?;

        file.write_all(&bytes)?;

        println!(
            "Generated {} games with {} enemies each: {}",
            args.games,
            args.enemies,
            path.display()
        );
    }

    let bytes = fs::read(&path)?;
    let workload: RealtimeWorkload<'_> = serde_json::from_slice(&bytes)?;

    workload.validate()?;

    if args.generate_only {
        return Ok(());
    }

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!("perf/results/realtime-{timestamp}.json")));

    if output.exists() {
        return Err(format!(
            "result already exists: {}; choose a new --output",
            output.display()
        )
        .into());
    }

    // Validate every corpus member and fingerprint outputs before the timed loop.
    let mut result_hash = 0u64;

    for game in &workload.games {
        let result = tutorlolv2::realtime::realtime(game).map_err(|error| error.to_string())?;
        result_hash = result_hash.rotate_left(7) ^ fingerprint(&serde_json::to_vec(&result)?);
    }

    if cfg!(debug_assertions) {
        eprintln!(
            "Warning: unoptimized/debug run; use cargo perf --realtime for performance measurements."
        );
    }

    let measurement = run_realtime(
        &workload.games,
        Settings {
            iterations: args.iterations,
            samples: args.samples,
            warmup: args.warmup,
        },
    )?;

    let executable_hash = fingerprint(&fs::read(std::env::current_exe()?)?);
    let cpu = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines().find_map(|line| {
                line.strip_prefix("model name")
                    .and_then(|s| s.split_once(':'))
                    .map(|(_, value)| value.trim().to_owned())
            })
        })
        .or_else(|| std::env::var("PROCESSOR_IDENTIFIER").ok());

    let report = json!({
        "schema_version": 1,
        "function": "realtime",
        "timestamp_unix_ns": timestamp.to_string(),
        "timing_scope": "realtime call + black_box + timer overhead; excludes output drop, corpus selection, JSON, preparation, validation, warmup, reporting and compilation",
        "timer_policy": "1000 empty clock pairs measured separately; overhead reported without subtraction",
        "percentile_scope": "p95 of per-batch average ns/call, not individual-call tail latency",
        "workload": {
            "path": path, "fingerprint_fnv1a64": format!("{:016x}", fingerprint(&bytes)),
            "generator": workload.generator, "seed": workload.seed, "games": workload.games.len(),
            "enemies_per_game": workload.enemies_per_game, "allies_per_game": workload.allies_per_game,
            "simulated_items_per_enemy": tutorlolv2::ItemId::L_SIML,
            "output_fingerprint_fnv1a64": format!("{result_hash:016x}")
        },
        "build": {
            "rustc": env!("PERF_RUSTC"), "target": env!("PERF_TARGET"), "opt_level": env!("PERF_OPT_LEVEL"),
            "rustflags": env!("PERF_RUSTFLAGS"), "debug_assertions": cfg!(debug_assertions),
            "executable_fingerprint_fnv1a64": format!("{executable_hash:016x}"),
            "git_head_at_run": command("git", &["rev-parse", "HEAD"]),
            "git_status_at_run": command("git", &["status", "--short"]),
        },
        "host": { "cpu": cpu, "os": std::env::consts::OS, "arch": std::env::consts::ARCH,
            "logical_cpus": std::thread::available_parallelism().map(|n| n.get()).ok() },
        "measurement": measurement,
    });

    parent(&output)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)?;

    serde_json::to_writer_pretty(&mut file, &report)?;

    file.write_all(b"\n")?;

    println!(
        "{} calls: mean {:.3} us, median {:.3} us, p95 batch mean {:.3} us; {:.0} calls/sec",
        measurement.measured_calls,
        measurement.summary.mean_ns_per_call / 1000.0,
        measurement.summary.median_ns_per_call / 1000.0,
        measurement.summary.p95_batch_mean_ns_per_call / 1000.0,
        measurement.summary.calls_per_second
    );

    println!("Results: {}", output.display());

    Ok(())
}

fn main() {
    if let Err(error) = execute(Args::parse()) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
