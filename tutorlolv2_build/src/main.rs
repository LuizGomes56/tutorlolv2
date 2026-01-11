fn main() {
    let start = std::time::Instant::now();
    tutorlolv2_build::run().unwrap();
    println!("[time] fn run took: {end:?}", end = start.elapsed());
}
