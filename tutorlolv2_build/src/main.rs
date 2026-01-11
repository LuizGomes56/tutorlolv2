#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let start = std::time::Instant::now();
    tutorlolv2_build::run().await.unwrap();
    println!("[time] fn run took: {end:?}", end = start.elapsed());
}
