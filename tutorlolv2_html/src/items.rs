use crate::{html::Html, parallel_task};
use tutorlolv2_gen::{CastId, ItemId};

pub fn items_html() {
    parallel_task(|item_id: ItemId| {
        let mut html = Html::new(item_id);

        html.code(item_id.formula())
            .section("Source code definition for damage calculation")
            .code(item_id.closure())
            .describe()
            .idents(item_id.idents())
            .section("Item generator definition")
            .code(item_id.generator())
            .json(item_id);

        html
    });
}
