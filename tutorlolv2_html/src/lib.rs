use crate::{champions::champions_html, items::items_html, runes::runes_html};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;
use tutorlolv2_gen::CastId;

mod champions;
mod html;
mod items;
mod runes;

pub use html::Html;

pub(crate) fn parallel_task<F, T>(f: F)
where
    F: Fn(T) -> Html + Sync,
    T: CastId + Sync,
{
    let path = Path::new("__html");

    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }

    T::VALUES
        .into_par_iter()
        .try_for_each(|&value| {
            let html = f(value).into_inner();
            let folder = path.join(Html::folder(value));
            let target = folder.join(Html::file(value));

            std::fs::create_dir_all(&target)?;

            println!("[write] {target:?}");

            let result = tutorlolv2_fmt::minify_html(&html);

            let outputs = [
                (
                    target.join("index.zst"),
                    tutorlolv2_fmt::encode_zstd_9(&result),
                ),
                (
                    target.join("index.br"),
                    tutorlolv2_fmt::encode_brotli_11(&result),
                ),
                (target.join("index.html"), result),
            ];

            for (path, bytes) in outputs {
                std::fs::write(path, bytes)?;
            }

            std::io::Result::Ok(())
        })
        .unwrap();
}

pub fn run() {
    rayon::scope(|s| {
        [champions_html, items_html, runes_html]
            .into_iter()
            .for_each(|html| {
                s.spawn(move |_| html());
            });
    });
}
