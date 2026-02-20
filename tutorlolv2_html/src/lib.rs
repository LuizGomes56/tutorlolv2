use crate::{champions::champions_html, html::Html, items::items_html, runes::runes_html};
use std::{path::Path, sync::Arc};
use tokio::sync::Semaphore;
use tutorlolv2_gen::CastId;

pub mod champions;
pub mod html;
pub mod items;
pub mod runes;

pub async fn parallel_task<F, T, Fut>(permits: usize, f: F)
where
    Fut: Future<Output = Html> + Send,
    F: Copy + 'static + Send + Sync + Fn(T) -> Fut,
    T: Copy + CastId + Send + Sync + 'static,
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
