use crate::{html::Html, parallel_task};
use tutorlolv2_gen::runes::{RUNE_CLOSURES, RUNE_FORMULAS, RuneId};

pub fn runes_html() {
    parallel_task(|rune_id: RuneId| {
        let mut html = Html::new(rune_id);

        html.push_code_block(RUNE_FORMULAS[rune_id as usize].clone());
        html.push_code_block(RUNE_CLOSURES[rune_id as usize].clone());

        html
    });
}
