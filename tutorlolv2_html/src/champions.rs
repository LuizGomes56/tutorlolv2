use crate::{html::Html, parallel_task};
use tutorlolv2_exports::{CHAMPION_ABILITIES, CHAMPION_FORMULAS, CHAMPION_GENERATOR};
use tutorlolv2_gen::ChampionId;

pub async fn champions_html() {
    parallel_task(64, ChampionId::ARRAY, async |champion_id| {
        let number_of_abilities = champion_id.number_of_abilities();
        let mut html = Html::new(champion_id);

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
        html.push_code_block(main_offsets);
        html.push_str(&abilities);

        let generator_offsets = CHAMPION_GENERATOR[champion_id as usize];
        html.push_code_block(generator_offsets);
        html.push_json(champion_id).await;

        html.push_str(&format!(
            "This champion has {number_of_abilities} different damaging abilities"
        ));
        html
    })
    .await;
}
