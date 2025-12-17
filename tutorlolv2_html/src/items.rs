use crate::{html::Html, parallel_task};
use tutorlolv2_exports::{ITEM_FORMULAS, ITEM_GENERATOR};
use tutorlolv2_gen::ItemId as This;

pub async fn items_html() {
    parallel_task(64, This::ARRAY, |item_id| {
        let name = item_id.name();
        let mut html = Html::new(name);

        let main_offsets = ITEM_FORMULAS[item_id as usize];
        let main_code = Html::code_block(main_offsets);
        let generator_offsets = ITEM_GENERATOR[item_id as usize];
        let generator_code = Html::code_block(generator_offsets);

        html.push_str(&main_code);
        html.push_str(&generator_code);
        html
    })
    .await;
}
