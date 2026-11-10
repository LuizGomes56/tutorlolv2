use std::{
    ffi::{OsStr, OsString},
    path::Path,
};
use tutorlolv2_perf_runner::Runner;
#[test]
fn routes_realtime_and_preserves_arguments() {
    let runner =
        Runner::new(["--realtime", "--portable", "--input", "a b.json"].map(OsString::from));
    assert_eq!(
        runner.command().get_args().collect::<Vec<_>>(),
        [
            "run",
            "--profile",
            "perf",
            "-p",
            "tutorlolv2_perf",
            "--bin",
            "realtime",
            "--",
            "--input",
            "a b.json"
        ]
        .map(OsStr::new)
    );
    assert_eq!(runner.flags(None, None), None);
}
#[test]
fn native_flags_preserve_encoded_precedence() {
    let runner = Runner::new([]);
    assert_eq!(
        runner.flags(Some("-C\x1fopt-level=2".into()), Some("ignored".into())),
        Some((
            "CARGO_ENCODED_RUSTFLAGS",
            "-C\x1fopt-level=2\x1f-C\x1ftarget-cpu=native".into()
        ))
    );
    assert_eq!(
        runner.flags(None, Some("--cfg example".into())),
        Some(("RUSTFLAGS", "--cfg example -C target-cpu=native".into()))
    );
    assert_eq!(
        runner.flags(Some(OsString::new()), None),
        Some(("CARGO_ENCODED_RUSTFLAGS", "-C\x1ftarget-cpu=native".into()))
    );
}
#[test]
fn portable_leaves_environment_untouched() {
    let runner = Runner::new(["--portable".into()]);
    assert_eq!(
        runner.flags(Some("encoded".into()), Some("plain".into())),
        None
    );
}
#[cfg(unix)]
#[test]
fn forwards_non_utf8_paths_and_flags_without_loss() {
    use std::os::unix::ffi::OsStringExt;
    let path = OsString::from_vec(b"fixture-\xff.json".to_vec());
    let runner = Runner::new(["--input".into(), path.clone()]);
    assert_eq!(runner.command().get_args().last(), Some(path.as_os_str()));
    let flags = OsString::from_vec(b"--cfg=\xff".to_vec());
    let mut expected = flags.clone();
    expected.push("\x1f-C\x1ftarget-cpu=native");
    assert_eq!(
        runner.flags(Some(flags), None),
        Some(("CARGO_ENCODED_RUSTFLAGS", expected))
    );
}
#[test]
fn command_uses_workspace_and_perf_profile() {
    let command = Runner::new(["--games".into(), "1".into()]).command();
    assert_eq!(
        command.get_current_dir(),
        Some(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap()
        )
    );
    assert_eq!(
        command.get_args().collect::<Vec<_>>(),
        [
            "run",
            "--profile",
            "perf",
            "-p",
            "tutorlolv2_perf",
            "--bin",
            "tutorlolv2_perf",
            "--",
            "--games",
            "1"
        ]
        .map(OsStr::new)
    );
}
