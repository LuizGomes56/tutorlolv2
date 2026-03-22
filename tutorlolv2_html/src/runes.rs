use crate::{html::Html, parallel_task};
use tutorlolv2_gen::{CastId, RuneId};

pub fn runes_html() {
    parallel_task(|rune_id: RuneId| {
        let mut html = Html::new(rune_id);

        html.code(rune_id.formula())
            .section("Source code definition for damage calculation")
            .code(rune_id.closure())
            .describe()
            .idents(rune_id.idents())
            .json(rune_id);

        html
    });
}
