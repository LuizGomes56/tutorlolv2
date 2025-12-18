use crate::{html::Html, parallel_task};
use tutorlolv2_exports::{ITEM_FORMULAS, ITEM_GENERATOR};
use tutorlolv2_gen::ItemId;

pub async fn items_html() {
    parallel_task(64, ItemId::ARRAY, async |item_id| {
        let mut html = Html::new(item_id);

        let main_offsets = ITEM_FORMULAS[item_id as usize];
        html.push_code_block(main_offsets);

        let generator_offsets = ITEM_GENERATOR[item_id as usize];
        html.push_code_block(generator_offsets);
        html.push_json(item_id).await;
        html
    })
    .await;
}
