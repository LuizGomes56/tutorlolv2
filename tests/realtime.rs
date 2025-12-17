pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;

const CFG: bincode::config::Configuration = bincode::config::standard();

#[test]
fn test_realtime() -> MayFail {
    let bytes = std::fs::read("temp_test.json")?;
    let game = serde_json::from_slice(&bytes)?;
    let data = tutorlolv2::realtime(&game).ok_or("Failed to run `tutorlolv2::realtime`")?;

    let dbg_data = format!("{data:#?}");
    let dbg_json = serde_json::to_string_pretty(&data)?;
    let dbg_bin = bincode::encode_to_vec(&data, CFG)?;

    std::fs::write("realtime_test.txt", dbg_data)?;
    std::fs::write("realtime_test.json", dbg_json)?;
    std::fs::write("realtime_test.bin", dbg_bin)?;
    Ok(())
}
