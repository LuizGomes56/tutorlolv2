use crate::{html::Html, parallel_task};
use tutorlolv2::{CastId, RuneId};

pub fn runes_html() {
    parallel_task(|rune_id: RuneId| {
        let mut html = Html::new(rune_id);

        html.code(rune_id.formula())
            .section("Source code definition for damage calculation");

        for range in rune_id.closure().iter().flatten() {
            html.code(range);
        }

        html.describe().idents(rune_id.identifiers()).json(rune_id);
        html
    });
}
