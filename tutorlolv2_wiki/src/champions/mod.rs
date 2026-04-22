use crate::client::MayFail;

pub mod abilities;
pub mod full;
pub mod template;

pub async fn run() -> MayFail {
    // full::download().await?;
    // full::parse()?;
    // template::download().await?;
    // template::parse()?;
    // abilities::download().await?;
    abilities::parse()?;

    Ok(())
}

pub fn clean_text(s: &str) -> String {
    s.replace('\u{a0}', " ")
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}
