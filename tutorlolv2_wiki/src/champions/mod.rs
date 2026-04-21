use crate::client::MayFail;

pub mod abilities;
pub mod full;
pub mod template;

pub async fn run() -> MayFail {
    full::download().await?;
    full::parse()?;
    template::download().await?;
    template::parse()?;

    Ok(())
}
