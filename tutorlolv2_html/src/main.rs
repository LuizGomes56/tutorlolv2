use core::ops::Range;
use tutorlolv2::{ChampionId, ItemId, RuneId, docs::DOCS};
use tutorlolv2_html::{Html, run};

fn main() {
    run();

    let source_code = |array: &[Range<usize>]| {
        array
            .iter()
            .map(|range| &DOCS[range.clone()])
            .collect::<String>()
    };

    let champions = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(&[champion_id.docs().clone()]))
        .collect::<String>();

    let items = ItemId::VALUES
        .into_iter()
        .map(|item_id| source_code(&[item_id.docs().clone()]))
        .collect::<String>();

    let runes = RuneId::VALUES
        .into_iter()
        .map(|rune_id| source_code(&[rune_id.docs().clone()]))
        .collect::<String>();

    let champion_gen = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(&[champion_id.generator_docs().clone()]))
        .collect::<String>();

    let item_gen = ItemId::VALUES
        .into_iter()
        .map(|item_id| source_code(&[item_id.generator_docs().clone()]))
        .collect::<String>();

    let rune_gen = RuneId::VALUES
        .into_iter()
        .map(|rune_id| source_code(&[rune_id.generator_docs().clone()]))
        .collect::<String>();

    let ability_closures = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(champion_id.functions_docs()))
        .collect::<String>();

    let items_closures = source_code(
        &ItemId::FUNCTIONS_DOCS
            .iter()
            .cloned()
            .flatten()
            .flatten()
            .collect::<Vec<_>>(),
    );

    let runes_closures = source_code(
        &RuneId::FUNCTIONS_DOCS
            .iter()
            .cloned()
            .flatten()
            .flatten()
            .collect::<Vec<_>>(),
    );

    let abilities = ChampionId::VALUES
        .into_iter()
        .map(|champion_id| source_code(champion_id.functions_docs()))
        .collect::<String>();

    let data = [
        champions,
        items,
        runes,
        champion_gen,
        item_gen,
        rune_gen,
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
