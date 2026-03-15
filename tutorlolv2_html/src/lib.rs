use crate::{champions::champions_html, html::Html, items::items_html, runes::runes_html};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;
use tutorlolv2_gen::CastId;

pub mod champions;
pub mod html;
pub mod items;
pub mod runes;

pub fn parallel_task<F, T>(f: F)
where
    F: Fn(T) -> Html + Sync,
    T: CastId + Sync,
{
    let path = Path::new("__html");

    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }

    T::VALUES.into_par_iter().for_each(|&value| {
        let html = f(value);
        let folder = path.join(Html::folder(value));

        if !folder.exists() {
            std::fs::create_dir_all(&folder).unwrap();
        }

        let target = folder.join(Html::file(value)).with_extension("html");

        println!("[write] {target:?}");

        let result = tutorlolv2_fmt::minify_html(&html.into_inner());
        std::fs::write(target, result).unwrap();
    });
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
