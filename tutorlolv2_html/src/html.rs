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

    pub fn section(&mut self, text: &str) -> &mut Self {
        self.push_str(&format!(
            r#"
            <h2 class="text-2xl text-std-200 font-medium">
                {text}
            </h2>
            "#
        ));
        self
    }

    pub fn describe(&mut self) -> &mut Self {
        self.push_str(&format!(
            r#"<p class="text-std-400">Requires knowledge of the following identifiers</p>"#
        ));
        self
    }

    pub fn add_footer(&mut self) {
        self.push_str("<footer>Automatically generated content</footer>");
    }

    pub fn get_json(enumv: impl CastId) -> Option<String> {
        let file = format!("{enumv:?}");
        let path = Path::new("internal")
            .join(Self::folder(enumv))
            .join(file)
            .with_extension("json");
        std::fs::read_to_string(&path)
            .ok()
            .as_ref()
            .map(String::as_str)
            .map(tutorlolv2_fmt::json_html)
    }

    pub fn json(&mut self, enumv: impl CastId) -> &mut Self {
        if let Some(json) = Self::get_json(enumv) {
            self.section(&format!("Json representation for {enumv:?}"))
                .code_block(&json)
                .push_str("</section>");
        }
        self
    }

    pub fn code_column(tag: &str, code: &str) -> String {
        format!(r#"<div class="column"><h2>{tag}</h2>{code}</div>"#)
    }

    pub fn idents(&mut self, array: &[CtxVar]) -> &mut Self {
        let conv = |ty: &str, text: &str| format!(r#"<span class="{ty}">{text}</span>"#);

        let idents = array
            .into_iter()
            .map(ToString::to_string)
            .map(|ctxvar| {
                let var = conv("variable", &ctxvar["ctx.".len()..]);
                format!("\t{var}")
            })
            .collect::<Vec<_>>()
            .join(",\n");

        let space = (!array.is_empty()).then_some("\n").unwrap_or_default();

        let code = format!(
            "<pre>{letkw} {ctxty} {lbracket}{space}{idents}{space}{rbracket} = {ctx};</pre>",
            letkw = conv("keyword", "let"),
            ctxty = conv("type", "Ctx"),
            lbracket = conv("bracket_1", "{"),
            rbracket = conv("bracket_1", "}"),
            ctx = conv("variable", "ctx")
        );

        self.code_block(&code)
    }

    pub fn code_block(&mut self, code: &str) -> &mut Self {
        self.push_str(&format!(
            r#"
            <code class="px-4 py-3 border border-std-800">
                {code}
            </code>
            "#
        ));
        self
    }

    pub fn code(&mut self, offsets: &Range<usize>) -> &mut Self {
        let code = &RAW_BLOCK[offsets.clone()];
        self.code_block(code)
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

    pub fn img_src(rest: &str) -> String {
        format!("{url}/img/{rest}", url = Self::URL)
    }

    pub fn src(value: impl CastId) -> String {
        let folder = Self::folder(value);
        let tag = match value.entity() {
            EntityId::Champion(champion_id) => format!("{champion_id:?}"),
            EntityId::Item(item_id) => item_id.to_riot_id().to_string(),
            EntityId::Rune(rune_id) => rune_id.to_riot_id().to_string(),
        };

        Self::img_src(&format!("{folder}/{tag}.avif"))
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
            <div class="flex flex-col gap-6 py-4 px-6 box max-w-6xl place-self-center w-full">
                <div class="flex items-center gap-4">
                    <img class="w-12 h-12" src="{src}" alt="{name}">
                    <span class="text-3xl font-medium">
                        {name}
                    </span>
                </div>
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
