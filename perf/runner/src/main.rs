use std::{env, process};
use tutorlolv2_perf_runner::Runner;

fn main() {
    process::exit(Runner::new(env::args_os().skip(1)).run());
}
