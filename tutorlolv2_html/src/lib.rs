use crate::{champions::champions_html, html::Html, items::items_html, runes::runes_html};
use core::{any::Any, fmt::Debug};
use std::{path::Path, sync::Arc};
use tokio::sync::Semaphore;

pub mod champions;
pub mod html;
pub mod items;
pub mod runes;

pub trait ArrayItem: Debug + Any + Sized + Copy + Into<usize> + Into<&'static str> {
    const ARRAY: &'static [Self];
    fn folder(&self) -> &'static str;
    fn file(&self) -> String {
        format!("{self:?}")
    }
    fn name(&self) -> &'static str {
        (*self).into()
    }
    fn offset(&self) -> usize {
        (*self).into()
    }
}

macro_rules! impl_array_item {
    ($($enum:tt),*) => {
        $(
            impl ArrayItem for tutorlolv2_gen::$enum {
                const ARRAY: &'static [Self] = &Self::ARRAY;
                fn folder(&self) -> &'static str {
                    pastey::paste! {
                        stringify!([<$enum:replace("Id", "s"):lower>])
                    }
                }
            }
        )*
    };
}
impl_array_item!(ChampionId, ItemId, RuneId);

pub async fn parallel_task<F, T, Fut>(permits: usize, f: F)
where
    Fut: Future<Output = Html> + Send,
    F: Copy + 'static + Send + Sync + Fn(T) -> Fut,
    T: Copy + ArrayItem + Send + Sync + 'static,
{
    let mut futures = Vec::new();
    let semaphore = Arc::new(Semaphore::new(permits));

    let path = Path::new("__html");

    if !path.exists() {
        tokio::fs::create_dir_all(path).await.unwrap();
    }

    for &value in T::ARRAY {
        let semaphore = semaphore.clone();
        futures.push(tokio::task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let html = f(value).await;
            let folder = path.join(value.folder());

            if !folder.exists() {
                tokio::fs::create_dir_all(&folder).await.unwrap();
            }

            let target = folder.join(value.file()).with_extension("html");

            println!("[write] {target:?}");

            tokio::fs::write(target, html.into_inner()).await.unwrap();
        }));
    }

    for future in futures {
        future.await.unwrap();
    }
}

pub async fn run() {
    for future in [
        tokio::spawn(champions_html()),
        tokio::spawn(items_html()),
        tokio::spawn(runes_html()),
    ] {
        future.await.unwrap();
    }
}
