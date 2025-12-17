use crate::{champions::champions_html, html::Html, items::items_html, runes::runes_html};
use std::{path::Path, sync::Arc};
use tokio::sync::Semaphore;

pub mod champions;
pub mod html;
pub mod items;
pub mod runes;

pub trait ArrayItem {
    fn folder(&self) -> &'static str;
    fn file(&self) -> String;
}

macro_rules! impl_array_item {
    ($($enum:ty),*) => {
        pastey::paste! {
            $(
                impl ArrayItem for tutorlolv2_gen::$enum {
                    fn folder(&self) -> &'static str {
                        stringify!([<$enum:replace("Id", "s"):lower>])
                    }
                    fn file(&self) -> String {
                        format!("{self:?}")
                    }
                }
            )*
        }
    };
}

impl_array_item!(ChampionId, ItemId, RuneId);

pub async fn parallel_task<F, A, T, Fut>(permits: usize, array: A, f: F)
where
    A: IntoIterator<Item = T>,
    Fut: Future<Output = Html> + Send,
    F: Copy + 'static + Send + Sync + Fn(T) -> Fut,
    T: Copy + ArrayItem + Send + Sync + 'static,
{
    let mut futures = Vec::new();
    let semaphore = Arc::new(Semaphore::new(permits));

    let path = Path::new("__html");

    for value in array {
        let semaphore = semaphore.clone();
        futures.push(tokio::task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let html = f(value).await;
            let folder = path.join(value.folder());

            let target = folder.join(value.file()).with_extension("html");
            tokio::fs::write(target, html.into_inner()).await.unwrap();
        }));
    }

    for future in futures {
        future.await.unwrap();
    }
}

pub async fn run() {
    champions_html().await;
    items_html().await;
    runes_html().await;
}
