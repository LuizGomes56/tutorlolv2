use crate::{html::Html, parallel_task};
use tutorlolv2_gen::items::{ITEM_CLOSURES, ITEM_FORMULAS, ITEM_GENERATOR, ItemId};

pub fn items_html() {
    parallel_task(|item_id: ItemId| {
        let mut html = Html::new(item_id);

        html.push_code_block(ITEM_FORMULAS[item_id as usize].clone());
        html.push_code_block(ITEM_GENERATOR[item_id as usize].clone());
        html.push_code_block(ITEM_CLOSURES[item_id as usize].clone());
        html.push_json(item_id);

        html
    });
}
