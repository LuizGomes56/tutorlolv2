use crate::{html::Html, parallel_task};
use tutorlolv2_exports::RUNE_FORMULAS;
use tutorlolv2_gen::RuneId as This;

pub async fn runes_html() {
    parallel_task(64, This::ARRAY, async |rune_id| {
        let name = rune_id.name();
        let mut html = Html::new(name);

        let main_offsets = RUNE_FORMULAS[rune_id as usize];
        let main_code = Html::code_block(main_offsets);

        html.push_str(&main_code);
        html
    })
    .await;
}
