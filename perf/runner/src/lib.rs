use std::{env, ffi::OsString, path::Path, process::Command};

/// Builds and runs the optimized harness while preserving native OS arguments.
pub struct Runner {
    binary: &'static str,
    portable: bool,
    args: Vec<OsString>,
}

impl Runner {
    pub fn new(args: impl IntoIterator<Item = OsString>) -> Self {
        let mut runner = Self {
            binary: "tutorlolv2_perf",
            portable: false,
            args: Vec::new(),
        };

        for arg in args {
            if arg == "--realtime" {
                runner.binary = "realtime";
            } else if arg == "--portable" {
                runner.portable = true;
            } else {
                runner.args.push(arg);
            }
        }

        runner
    }

    pub fn flags(
        &self,
        encoded: Option<OsString>,
        plain: Option<OsString>,
    ) -> Option<(&'static str, OsString)> {
        if self.portable {
            return None;
        }

        let (key, mut flags, separator) = match encoded {
            Some(flags) => ("CARGO_ENCODED_RUSTFLAGS", flags, "\x1f"),
            None => ("RUSTFLAGS", plain.unwrap_or_default(), " "),
        };

        if !flags.is_empty() {
            flags.push(separator);
        }

        flags.push("-C");
        flags.push(separator);
        flags.push("target-cpu=native");

        Some((key, flags))
    }

    pub fn command(&self) -> Command {
        let mut command = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));

        command
            .current_dir(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap(),
            )
            .args([
                "run",
                "--profile",
                "perf",
                "-p",
                "tutorlolv2_perf",
                "--bin",
                self.binary,
                "--",
            ])
            .args(&self.args);

        if let Some((key, flags)) = self.flags(
            env::var_os("CARGO_ENCODED_RUSTFLAGS"),
            env::var_os("RUSTFLAGS"),
        ) {
            command.env(key, flags);
        }

        command
    }

    #[allow(clippy::needless_return)]
    pub fn run(&self) -> i32 {
        match self.command().status() {
            Ok(status) => {
                if let Some(code) = status.code() {
                    return code;
                }

                #[cfg(unix)]
                {
                    use std::os::unix::process::ExitStatusExt;
                    return 128 + status.signal().unwrap_or(1);
                }

                #[cfg(not(unix))]
                1
            }
            Err(error) => {
                eprintln!("Could not launch performance harness: {error}");
                1
            }
        }
    }
}
