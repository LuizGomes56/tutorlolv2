use crate::{html::Html, parallel_task};
use tutorlolv2_gen::{ITEM_FORMULAS, ITEM_GENERATOR, ItemId};

pub async fn items_html() {
    parallel_task(64, async |item_id: ItemId| {
        let mut html = Html::new(item_id);

        html.push_code_block(ITEM_FORMULAS[item_id as usize]);
        html.push_code_block(ITEM_GENERATOR[item_id as usize]);
        html.push_json(item_id).await;
        html
    })
    .await;
}
