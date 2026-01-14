use tutorlolv2::ChampionId;

#[test]
pub fn test_calculator() {
    let name = "XinZhao";
    let serialized = serde_json::to_string(&ChampionId::XinZhao).unwrap();
    println!("Serialized: {serialized}");
    serde_json::from_str::<ChampionId>(&serialized).unwrap();
    serde_json::from_str::<ChampionId>(name).unwrap();
}
