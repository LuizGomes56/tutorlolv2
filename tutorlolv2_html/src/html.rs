use core::ops::{Deref, DerefMut};
use std::{fmt::Debug, ops::Range, path::Path};
use tutorlolv2_gen::{CastId, RAW_BLOCK, champions::ChampionId, items::ItemId};

pub struct Html {
    inner: String,
}

impl Html {
    const URL: &str = "http://localhost:8082";
    const CSS: &str = include_str!("style.css");

    pub fn into_inner(mut self) -> String {
        self.add_footer();
        let inner = self.inner;
        format!("{inner}</body></html>")
    }

    pub fn add_footer(&mut self) {
        self.push_str("<footer>Automatically generated content</footer>");
    }

    pub fn add_code(&mut self, offsets: Range<usize>) {
        self.push_str(&Self::code_block(offsets));
    }

    pub async fn push_json(&mut self, enumv: impl CastId + Debug) {
        let json = Self::json(&enumv).await;
        self.push_str(&format!(
            "<section>
                <h3>JSON-IR generated for {enumv:?}</h3>
                {json}
            </section>"
        ));
    }

    pub async fn json(enumv: &(impl CastId + Debug)) -> String {
        let file = format!("{enumv:?}");
        let path = Path::new("internal")
            .join(enumv.folder())
            .join(file)
            .with_extension("json");
        let json = tokio::fs::read_to_string(path).await.unwrap();
        tutorlolv2_fmt::json_html(&json)
    }

    pub fn code_column(tag: &str, code: &str) -> String {
        format!(r#"<div class="column"><h2>{tag}</h2>{code}</div>"#)
    }

    pub fn push_idents(&mut self, _idents: &[EvalIdent]) {
        todo!()
    }

    pub fn push_code_block(&mut self, offsets: Range<usize>) {
        let code = Self::code_block(offsets);
        self.push_str(&format!(
            "<section>
                <h3>Source code representation</h3>
                {code}
            </section>"
        ));
    }

    pub fn code_block(offsets: Range<usize>) -> String {
        let code = &RAW_BLOCK[offsets];
        format!("<pre><code>{code}</code></pre>")
    }

    pub fn redirect(value: impl CastId) -> String {
        let folder = value.folder();
        let file = value.file();
        let url = Self::URL;
        format!("{url}/{folder}/{file}.html")
    }

    pub fn src(value: impl CastId) -> String {
        let folder = value.folder();
        let type_id = value.type_id();
        let tag = match ChampionId::is(&type_id) {
            true => value.file(),
            false => match ItemId::is(&type_id) {
                true => ITEM_ID_TO_RIOT_ID[value.offset()],
                false => RUNE_ID_TO_RIOT_ID[value.offset()],
            }
            .to_string(),
        };

        let url = Self::URL;
        format!("{url}/img/{folder}/{tag}.avif")
    }

    pub fn new(value: impl CastId) -> Self {
        let name = value.name();
        let src = Self::src(value);
        let css = Html::CSS;
        let inner = format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{name}</title>
            </head>
            <style>{css}</style>
            <body>
            <header>
                <img src="{src}" alt="{name}">
                <h1>{name}</h1>
            </header>
            "#
        );
        Self { inner }
    }
}

impl Deref for Html {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Html {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
