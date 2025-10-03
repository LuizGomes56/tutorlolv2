use bumpalo::Bump;

fn main() {
    let arena = Bump::with_capacity(4 * 1024 * 1024);
    let bytes = std::fs::read("serde_test.json").expect("read serde_test.json");
    let game = serde_json::from_slice(&bytes).expect("JSON inválido em serde_test.json");
    let ptr = tutorlolv2_math::__v2::rt_arena::realtime_arena(&arena, &game).expect("err");
    println!("ptr: {:p}", ptr);
}
