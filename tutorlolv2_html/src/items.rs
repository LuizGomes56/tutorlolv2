use crate::{html::Html, parallel_task};
use tutorlolv2::ItemId;

pub fn items_html() {
    parallel_task(|item_id: ItemId| {
        let mut html = Html::new(item_id);

        html.code(item_id.docs())
            .section("Source code definition for damage calculation");

        for range in item_id.functions_docs().iter().flatten() {
            html.code(range);
        }

        html.describe()
            .idents(item_id.identifiers())
            .section("Item generator definition")
            .code(item_id.generator_docs())
            .json(item_id);

        html
    });
}
