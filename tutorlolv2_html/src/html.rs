use std::ops::{Deref, DerefMut};

use tutorlolv2_exports::RAW_BLOCK;

pub struct Html {
    inner: String,
}

impl Html {
    const CSS: &str = include_str!("style.css");

    pub fn into_inner(mut self) -> String {
        self.add_footer();
        let inner = self.inner;
        format!("{inner}</body></html>")
    }

    pub fn add_footer(&mut self) {
        self.push_str("<footer>Automatically generated content</footer>");
    }

    pub fn add_code(&mut self, offsets: (u32, u32)) {
        self.push_str(&Self::code_block(offsets));
    }

    pub fn code_column(tag: &str, code: &str) -> String {
        format!(r#"<div class="column"><h2>{tag}</h2>{code}</div>"#)
    }

    pub fn code_block(offsets: (u32, u32)) -> String {
        let (i, j) = offsets;
        let code = &RAW_BLOCK[i as _..j as _];
        format!("<pre><code>{code}</code></pre>")
    }

    pub fn new(title: &str) -> Self {
        let css = Html::CSS;
        let inner = format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{title}</title>
            </head>
            <style>{css}</style>
            <body>
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
