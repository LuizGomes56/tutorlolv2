use core::ops::{Deref, DerefMut};
use std::{ops::Range, path::Path};
use tutorlolv2_gen::{CastId, CtxVar, EntityId, RAW_BLOCK};

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

    pub fn push_json(&mut self, enumv: impl CastId) {
        let json = Self::json(enumv);
        self.push_str(&format!(
            "<section>
                <h3>JSON-IR generated for {enumv:?}</h3>
                {json}
            </section>"
        ));
    }

    pub fn json(enumv: impl CastId) -> String {
        let file = format!("{enumv:?}");
        let path = Path::new("internal")
            .join(Self::folder(enumv))
            .join(file)
            .with_extension("json");
        let json = std::fs::read_to_string(path).unwrap();
        tutorlolv2_fmt::json_html(&json)
    }

    pub fn code_column(tag: &str, code: &str) -> String {
        format!(r#"<div class="column"><h2>{tag}</h2>{code}</div>"#)
    }

    pub fn push_idents(&mut self, _idents: &[CtxVar]) {
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
        let folder = Self::folder(value);
        let url = Self::URL;
        let file = Self::file(value);
        format!("{url}/{folder}/{file}.html")
    }

    pub fn folder(value: impl CastId) -> &'static str {
        match value.entity() {
            EntityId::Champion(_) => "champions",
            EntityId::Item(_) => "items",
            EntityId::Rune(_) => "runes",
        }
    }

    pub fn file(value: impl CastId) -> String {
        format!("{value:?}")
    }

    pub fn src(value: impl CastId) -> String {
        let folder = Self::folder(value);
        let tag = match value.entity() {
            EntityId::Champion(champion_id) => format!("{champion_id:?}"),
            EntityId::Item(item_id) => item_id.to_riot_id().to_string(),
            EntityId::Rune(rune_id) => rune_id.to_riot_id().to_string(),
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
