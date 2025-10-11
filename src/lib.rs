#![allow(ambiguous_glob_reexports)]
#[cfg(feature = "dev")]
pub use tutorlolv2_dev::*;
pub use tutorlolv2_gen::*;
pub use tutorlolv2_math::*;

#[cfg(test)]
mod tests {
    use std::{
        process::{Child, Command, Stdio},
        thread::{sleep, spawn},
        time::Duration,
    };
    use tutorlolv2_exports::*;

    #[test]
    fn generate_html() {
        let champions = spawn(generate_champion_html);
        let items = spawn(generate_item_html);
        let runes = spawn(generate_rune_html);

        champions.join().unwrap();
        items.join().unwrap();
        runes.join().unwrap();
    }

    #[test]
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
            sleep(Duration::from_millis(5200));
        }

        run(".", "cargo", &["build", "-r"]);
        let srv_0 = task(".", "cargo", &["run", "-r"]);
        short_wait();
        http_get("http://localhost:8082/api/update/version");
        kill(srv_0);
        let srv_1 = task(".", "cargo", &["run", "-r"]);
        http_get("http://localhost:8082/api/setup/project");
        http_get("http://localhost:8082/api/images/compress");
        kill(srv_1);
        run("tutorlolv2_build", "cargo", &["build", "-r"]);
        run("tutorlolv2_build", "cargo", &["run", "-r"]);

        let srv_2 = task(".", "cargo", &["run", "-r"]);
        short_wait();
        http_get("http://localhost:8082/api/setup/docs");
        kill(srv_2);
        run(".", "cargo", &["build", "-r"]);
        run(".", "cargo", &["run", "-r", "--no-default-features"]);

        println!("Setup finished");
    }
}
