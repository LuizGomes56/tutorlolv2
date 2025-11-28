use std::{
    collections::HashMap,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};
use tutorlolv2_dev::{
    DeserializeOwned, JsonRead, Serialize,
    champions::MerakiChampion,
    client::{OrderJson, save_cache},
    gen_factories::fac_items::ItemFactory,
    generators::gen_factories::fac_champions::ChampionFactory,
    items::MerakiItem,
};

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
    let server = task(
        ".",
        "./tutorlolv2_server/target/release/tutorlolv2_server.exe",
        &[],
    );
    short_wait();
    server
}

macro_rules! get {
    ($path:literal) => {
        http_get(concat!("http://localhost:8082/api", $path));
    };
}

/// Requires `lolstaticdata` to be installed and placed in the parent directory
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

/// Updates local files only. Requires `tutorlolv2_desktop_app` to be installed and places in the parent directory,
/// containing `tutorlolv2_frontend` and the javascript build script.
/// Pulls champions and items data and generates intermediary JSON files and call the subsequent tasks to
/// process the output and generate Rust code to `tutorlolv2_gen`. Only works on Windows.
fn update_local() {
    build_server();
    let srv_0 = run_server();

    get!("/update/version");
    kill(srv_0);

    let srv_1 = run_server();

    get!("/setup/project");
    kill(srv_1);

    build_script();

    let srv_2 = run_server();

    get!("/setup/docs");
    run(
        "../tutorlolv2_desktop_app/tutorlolv2_frontend",
        "node",
        &["build_script.js"],
    );
    kill(srv_2);
    build_server();

    println!("Local finished");
}

fn build_script() {
    run("./tutorlolv2_build", "cargo", &["build", "-r"]);
    run("./tutorlolv2_build", "cargo", &["run", "-r"])
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
fn update() {
    build_server();
    let srv_0 = run_server();
    get!("/update/version");
    kill(srv_0);

    let srv_1 = run_server();
    get!("/setup/project");
    get!("/images/compress");
    kill(srv_1);

    build_script();

    let srv_2 = run_server();
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
        Some("build") => build_script(),
        Some("cdn-ord") => {
            async fn ord_folder<T>(endpoint: &str)
            where
                T: DeserializeOwned + Serialize,
                HashMap<String, T>: OrderJson<T>,
            {
                let _ = save_cache(
                    T::from_dir(format!("cache/meraki/{endpoint}")).unwrap(),
                    endpoint,
                );
            }

            ord_folder::<MerakiChampion>("champions").await;
            ord_folder::<MerakiItem>("items").await;
        }
        Some("check-gen") => ChampionFactory::check_all_offsets(),
        Some("item-gen") => ItemFactory::run_all().unwrap(),
        Some("champion-gen") => ChampionFactory::run_all().unwrap(),
        Some("make-gen") => {
            let _ = ChampionFactory::create_all();
        }
        Some("html-gen") => {
            build_server();
            let srv_0 = run_server();
            get!("/setup/docs");
            kill(srv_0);
        }
        Some("update") => update(),
        Some("lolstaticdata") => run_lolstaticdata(),
        Some("local") => update_local(),
        _ => tutorlolv2_server::run().await.unwrap(),
    }
}
