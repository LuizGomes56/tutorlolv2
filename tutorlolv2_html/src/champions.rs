use tutorlolv2_gen::{
    Position,
    champions::{
        ABILITY_CLOSURES, ABILITY_FORMULAS, CHAMPION_FORMULAS, CHAMPION_GENERATOR, ChampionId,
        RECOMMENDED_ITEMS, RECOMMENDED_RUNES,
    },
};

use crate::{ArrayItem, html::Html, parallel_task};

fn get_recommendations<T: ArrayItem, const N: usize, const M: usize>(
    champion_id: ChampionId,
    array: &'static [[&[T]; N]; M],
) -> String {
    array[champion_id as usize]
        .into_iter()
        .enumerate()
        .map(|(i, values)| unsafe {
            let position = Position::from_u8_unchecked(i as _);
            let items = values
                .into_iter()
                .map(|value| {
                    let src = Html::src(*value);
                    let name = value.name();
                    format!(
                        r#"<li>
                            <img src="{src}" alt="{name}">
                            <span>[{position:?}] {value:?}</span>
                        </li>"#
                    )
                })
                .collect::<String>();
            format!("<ul>{items}</ul>")
        })
        .collect::<String>()
}

pub async fn champions_html() {
    parallel_task(64, async |champion_id: ChampionId| {
        let number_of_abilities = champion_id.number_of_abilities();
        let mut html = Html::new(champion_id);

        let positions = champion_id
            .cache()
            .positions
            .into_iter()
            .map(|position| format!("<li>{position:?}</li>"))
            .collect::<String>();

        let item_recommendations = get_recommendations(champion_id, &RECOMMENDED_ITEMS);
        let rune_recommendations = get_recommendations(champion_id, &RECOMMENDED_RUNES);

        let abilities = champion_id
            .cache()
            .metadata
            .into_iter()
            .enumerate()
            .map(|(i, metadata)| {
                let const_lit = metadata.kind.as_const_lit();
                let lit = const_lit.trim_end_matches(".cast()");
                let full_code = Html::code_block(ABILITY_FORMULAS[champion_id as usize][i].clone());
                let part_code = Html::code_block(ABILITY_CLOSURES[champion_id as usize][i].clone());
                Html::code_column(lit, &full_code) + &Html::code_column(lit, &part_code)
            })
            .collect::<String>();

        html.push_str(&format!(
            "<div>
                <h3>This champion commonly plays in the following positions</h3>
                <ul>{positions}</ul>
            </div>"
        ));
        html.push_str(&item_recommendations);
        html.push_str(&rune_recommendations);
        html.push_code_block(CHAMPION_FORMULAS[champion_id as usize].clone());
        html.push_str(&abilities);
        html.push_code_block(CHAMPION_GENERATOR[champion_id as usize].clone());
        html.push_json(champion_id).await;

        html.push_str(&format!(
            "This champion has {number_of_abilities} different damaging abilities"
        ));
        html
    })
    .await;
}
