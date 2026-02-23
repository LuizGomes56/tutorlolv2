use core::ops::Range;
use tutorlolv2::{
    ChampionId, ItemId, RuneId, champions::ABILITY_CLOSURES, items::ITEM_CLOSURES,
    runes::RUNE_CLOSURES,
};
use tutorlolv2_gen::{CastId, RAW_BLOCK};

const CSS: &str = r###"body{background-color:#1F1F1F;}.punctuation {color: #D4D4D4;}.control {color: #C586C0;}.lifetime, .keyword, .macro, .boolean {color: #569CD6;}.primitive, .type {color: #4EC8B0;}.comment {color: #959596;}.function {color: #DCDCAA;}.number, .float {color: #B3CDA8;}.constant, .intrinsic {color: #4FC1FF;}.bracket_1 {color: #FFD700;}.bracket_2 {color: #DA70D6;}.bracket_3 {color: #189FFF;}.string {white-space: break-spaces;color: #CE9178;}.variable {color: #9CDCFE;}code, pre {color: #D4D4D4;line-height: 1.5;background: transparent;font-family: Consolas, Monaco, 'AndaleMono', 'UbuntuMono', monospace;}"###;

#[test]
fn render() {
    let source_code = |array: &[Range<usize>]| {
        array
            .iter()
            .map(|range| RAW_BLOCK[range.clone()].to_string())
            .collect::<String>()
    };

    let champions = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(&[champion_id.formula().clone()]))
        .collect::<String>();

    let items = ItemId::VALUES
        .into_iter()
        .map(|item_id| source_code(&[item_id.formula().clone()]))
        .collect::<String>();

    let runes = RuneId::VALUES
        .into_iter()
        .map(|rune_id| source_code(&[rune_id.formula().clone()]))
        .collect::<String>();

    let champion_gen = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(&[champion_id.generator().clone()]))
        .collect::<String>();

    let item_gen = ItemId::VALUES
        .into_iter()
        .map(|item_id| source_code(&[item_id.generator().clone()]))
        .collect::<String>();

    let ability_closures = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(ABILITY_CLOSURES[champion_id.index()]))
        .collect::<String>();
    let items_closures = source_code(&ITEM_CLOSURES);
    let runes_closures = source_code(&RUNE_CLOSURES);

    let abilities = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(champion_id.ability_formulas()))
        .collect::<String>();

    let data = [
        champions,
        items,
        runes,
        champion_gen,
        item_gen,
        ability_closures,
        items_closures,
        runes_closures,
        abilities,
    ]
    .concat();

    std::fs::write(
        "temp.html",
        format!("<html><head><style>{CSS}</style></head><body>{data}</body></html>"),
    )
    .unwrap();
}
