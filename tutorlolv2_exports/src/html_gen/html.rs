use crate::MEGA_BLOCK;
use std::fmt::Debug;
use tutorlolv2_fmt::{encode_brotli_11, encode_zstd_9, highlight_json, minify_html, prettify_json};

pub const BASE_CSS: &'static str = include_str!("../../assets/base.css");

pub trait HtmlExt {
    fn code_section(&mut self, comment: &str, code: &str);
    fn finish(&mut self, source: Source, name: impl Debug);
    fn json_code(&self) -> String;
    fn footer(&mut self);
}

pub fn offset_to_str(offsets: (u32, u32)) -> &'static str {
    unsafe { MEGA_BLOCK.get_unchecked(offsets.0 as usize..offsets.1 as usize) }
}

pub enum Source {
    Champions,
    Items,
    Runes,
}

impl HtmlExt for String {
    fn json_code(&self) -> String {
        highlight_json(&prettify_json(
            &std::fs::read_to_string(self).unwrap_or_else(|_| "{}".to_string()),
        ))
    }

    fn code_section(&mut self, comment: &str, code: &str) {
        self.push_str(&format!(
            r#"<section style="background: #1a1a1a; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">{}</h2>
                <code>{}</code>
            </section>"#,
            comment,
            code
        ));
    }

    fn footer(&mut self) {
        self.push_str(r#"
            </main><footer style="text-align: center; padding: 20px; margin-top: 40px; color: #888; font-size: 0.9rem; border-top: 1px solid #333;">
            <p style="margin: 0;">Documentation automatically generated - tutorlolv2</p>
            </footer></div></body></html>"#
        );
    }

    fn finish(&mut self, source: Source, name: impl Debug) {
        let minified_html = minify_html(self);
        let zstd_bytes = encode_zstd_9(&minified_html);
        let brotli_bytes = encode_brotli_11(&minified_html);
        let path = format!(
            "{}/{:?}",
            match source {
                Source::Champions => "champions",
                Source::Items => "items",
                Source::Runes => "runes",
            },
            name
        );

        // Get-ChildItem -Path .\html -Recurse -File -Force | Remove-Item -Force
        unsafe {
            // .html extension omitted
            std::fs::write(format!("html/raw/{path}"), minified_html).unwrap_unchecked();
            // .br extension omitted
            std::fs::write(format!("html/brotli/{path}"), brotli_bytes).unwrap_unchecked();
            // .zst extension omitted
            std::fs::write(format!("html/zstd/{path}"), zstd_bytes).unwrap_unchecked();
        }
    }
}
