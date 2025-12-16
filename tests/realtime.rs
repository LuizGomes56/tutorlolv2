pub fn test_realtime() {
    let bytes = std::fs::read("temp_test.json").unwrap();
    let game = serde_json::from_slice(&bytes).unwrap();
    let data = tutorlolv2::realtime(&game).unwrap();
    std::fs::write("realtime_test.txt", format!("{data:#?}")).unwrap();
}
