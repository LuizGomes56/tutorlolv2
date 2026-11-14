fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        panic!("native requires --target x86_64-pc-windows-msvc");
    }
}
