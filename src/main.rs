use actix_web::rt::task::spawn_blocking;
use std::{
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};
use tutorlolv2_exports::*;

async fn generate_html() {
    let champions = spawn_blocking(generate_champion_html);
    let items = spawn_blocking(generate_item_html);
    let runes = spawn_blocking(generate_rune_html);

    champions.await.unwrap();
    items.await.unwrap();
    runes.await.unwrap();
}

fn run(cwd: &str, prog: &str, args: &[&str]) {
    let status = Command::new(prog)
        .current_dir(cwd)
        .args(args)
        .status()
        .expect("Command execution failed");
    assert!(status.success(), "{prog} {:?} exited with error", args);
}

fn task(cwd: &str, prog: &str, args: &[&str]) -> Child {
    Command::new(prog)
        .current_dir(cwd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Could not spawn child process")
}

fn kill(mut child: Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn http_get(url: &str) {
    let status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!("Invoke-WebRequest -UseBasicParsing '{}' | Out-Null", url),
        ])
        .status()
        .expect("Could not run PowerShell");
    assert!(status.success(), "Request GET {} failed", url);
}

fn short_wait() {
    thread::sleep(Duration::from_millis(10000));
}

fn build_server() {
    run("./tutorlolv2_server", "cargo", &["build", "-r"]);
}

fn run_server() -> Child {
    task(
        ".",
        "./tutorlolv2_server/target/release/tutorlolv2_server.exe",
        &[],
    )
}

macro_rules! get {
    ($path:literal) => {
        http_get(concat!("http://localhost:8082/api", $path));
    };
}

fn run_lolstaticdata() {
    run(
        "../lolstaticdata",
        "python",
        &["-m", "lolstaticdata.champions"],
    );
    run("../lolstaticdata", "python", &["-m", "lolstaticdata.items"]);

    run(
        ".",
        "powershell",
        &[
            "-NoProfile",
            "-Command",
            "$ErrorActionPreference='Stop'; New-Item -ItemType Directory -Force -Path .\\cache\\cdn, .\\cache\\cdn\\champions, .\\cache\\cdn\\items | Out-Null",
        ],
    );

    run(
        ".",
        "powershell",
        &[
            "-NoProfile",
            "-Command",
            "$ErrorActionPreference='Stop'; Copy-Item ..\\lolstaticdata\\champions\\* -Destination .\\cache\\cdn\\champions -Recurse -Force",
        ],
    );

    run(
        ".",
        "powershell",
        &[
            "-NoProfile",
            "-Command",
            "$ErrorActionPreference='Stop'; Copy-Item ..\\lolstaticdata\\items\\* -Destination .\\cache\\cdn\\items -Recurse -Force",
        ],
    );
}

/// Updates local files only. Requires `lolstaticdata` to be installed and places in the parent directory.
/// Same for `tutorlolv2_desktop_app`, containing `tutorlolv2_frontend` and the javascript build script.
/// Pulls champions and items data and generates intermediary JSON files and call the subsequent tasks to
/// process the output and generate Rust code to `tutorlolv2_gen`. Only works on Windows.
async fn update_local() {
    run_lolstaticdata();
    build_server();
    let srv_0 = run_server();
    short_wait();

    get!("/setup/champions");
    get!("/setup/items");
    kill(srv_0);

    build_script().await;

    let srv_1 = run_server();
    short_wait();

    get!("/setup/docs");
    run(
        "../tutorlolv2_desktop_app/tutorlolv2_frontend",
        "node",
        &["build_script.js"],
    );
    kill(srv_1);
    build_server();
    run(
        "./tutorlolv2_server",
        "cargo",
        &["build", "-r", "--no-default-features"],
    );

    println!("Local finished");
}

async fn build_script() {
    tutorlolv2::build::run().await;
}

/// Planned code task execution (in sequence, sync)
/// ```rs
/// ::task("cargo build -r");
/// ::task("cargo run -r");
/// ::task("http://localhost:8082/api/update/version");
/// ::task("kill");
/// ::task("cargo run -r");
/// ::task("http://localhost:8082/api/setup/project");
/// ::task("http://localhost:8082/api/images/compress");
/// ::task("kill");
/// ::task("cd tutorlolv2_build");
/// ::task("cargo build -r");
/// ::task("cargo run -r");
/// ::task("cd ..")
/// ::task("http://localhost:8082/api/setup/docs");
/// ::task("kill");
/// ::echo("Setup finished");
/// ```
async fn update() {
    build_server();
    let srv_0 = run_server();
    short_wait();
    get!("/update/version");
    kill(srv_0);
    let srv_1 = run_server();
    short_wait();
    get!("/setup/project");
    get!("/images/compress");
    kill(srv_1);

    build_script().await;

    let srv_2 = run_server();
    short_wait();
    get!("/setup/docs");
    kill(srv_2);
    build_server();
    run(
        "./tutorlolv2_server",
        "cargo",
        &["build", "-r", "--no-default-features"],
    );

    println!("Setup finished");
}

#[actix_web::main]
async fn main() {
    match std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map(String::as_str)
    {
        Some("-h") => generate_html().await,
        Some("-u") => update().await,
        Some("-l") => update_local().await,
        Some("-r") => {
            run(
                "./tutorlolv2_server",
                "cargo",
                &["build", "-r", "--no-default-features"],
            );
            run(
                "./tutorlolv2_server",
                "./target/release/tutorlolv2_server.exe",
                &[],
            )
        }
        _ => tutorlolv2_server::run().await.unwrap(),
    }
}
