use crate::{html::Html, parallel_task};
use tutorlolv2_gen::RuneId as This;

pub async fn runes_html() {
    parallel_task(64, This::ARRAY, |rune_id| {
        let name = rune_id.name();
        let mut html = Html::new(name);

        html
    })
    .await;
}
