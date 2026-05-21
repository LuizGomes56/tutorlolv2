use crate::{html::Html, parallel_task};
use tutorlolv2_gen::{CastId, RuneId};

pub fn runes_html() {
    parallel_task(|rune_id: RuneId| {
        let mut html = Html::new(rune_id);

        html.code(rune_id.formula())
            .section("Source code definition for damage calculation");

        for range in rune_id.closure().iter().flatten() {
            html.code(range);
        }

        html.describe();

        for array in rune_id.identifiers().iter().flatten() {
            html.idents(array);
        }

        html.json(rune_id);

        html
    });
}
