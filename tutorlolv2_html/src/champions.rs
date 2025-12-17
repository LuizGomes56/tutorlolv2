use crate::{html::Html, parallel_task};
use tutorlolv2_exports::{CHAMPION_ABILITIES, CHAMPION_FORMULAS, CHAMPION_GENERATOR};
use tutorlolv2_gen::ChampionId as This;

pub async fn champions_html() {
    parallel_task(64, This::ARRAY, async |champion_id| {
        let name = champion_id.name();
        let number_of_abilities = champion_id.number_of_abilities();
        let mut html = Html::new(name);

        let abilities = champion_id
            .get_cache()
            .metadata
            .into_iter()
            .enumerate()
            .map(|(i, metadata)| {
                let lit = metadata.kind.as_const_lit();
                let (_, offsets) = CHAMPION_ABILITIES[champion_id as usize][i];
                let code = Html::code_block(offsets);
                Html::code_column(&lit, &code)
            })
            .collect::<String>();

        let main_offsets = CHAMPION_FORMULAS[champion_id as usize];
        let main_code = Html::code_block(main_offsets);
        let generator_offsets = CHAMPION_GENERATOR[champion_id as usize];
        let generator_code = Html::code_block(generator_offsets);
        let json_code = Html::json(champion_id).await;

        html.push_str(&main_code);
        html.push_str(&abilities);
        html.push_str(&generator_code);
        html.push_str(&json_code);
        html.push_str(&format!(
            "This champion has {number_of_abilities} different damaging abilities"
        ));
        html
    })
    .await;
}
