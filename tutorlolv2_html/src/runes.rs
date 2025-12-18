use crate::{html::Html, parallel_task};
use tutorlolv2_exports::RUNE_FORMULAS;
use tutorlolv2_gen::RuneId;

pub async fn runes_html() {
    parallel_task(4, RuneId::ARRAY, async |rune_id| {
        let mut html = Html::new(rune_id);

        let main_offsets = RUNE_FORMULAS[rune_id as usize];
        html.push_code_block(main_offsets);
        html
    })
    .await;
}
