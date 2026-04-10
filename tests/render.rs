use core::ops::Range;
use tutorlolv2::{
    ChampionId, ItemId, RuneId, champions::ABILITY_CLOSURES, items::ITEM_CLOSURES,
    runes::RUNE_CLOSURES,
};
use tutorlolv2_gen::{CastId, RAW_BLOCK};
use tutorlolv2_html::Html;

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

    let css = Html::CSS;

    std::fs::write(
        "temp.html",
        format!("<html><head><style>{css}</style></head><body>{data}</body></html>"),
    )
    .unwrap();
}
