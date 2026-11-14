use perf_runner::Runner;
use std::{env, process};

fn main() {
    process::exit(Runner::new(env::args_os().skip(1)).run());
}
