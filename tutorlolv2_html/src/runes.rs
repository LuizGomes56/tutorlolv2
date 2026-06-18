use crate::{html::Html, parallel_task};
use tutorlolv2::RuneId;

pub fn runes_html() {
    parallel_task(|rune_id: RuneId| {
        let mut html = Html::new(rune_id);

        html.code(rune_id.docs())
            .section("Source code definition for damage calculation");

        for range in rune_id.functions_docs().iter().flatten() {
            html.code(range);
        }

        html.describe().idents(rune_id.identifiers()).json(rune_id);
        html
    });
}
