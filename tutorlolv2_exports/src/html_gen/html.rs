use tutorlolv2_fmt::{encode_zstd_9, minify_html};

pub const BASE_CSS: &'static str = include_str!("../../assets/champions.css");

pub trait HtmlExt {
    fn code_section(&mut self, comment: &str, code: &str);
    fn finish(&mut self) -> Vec<u8>;
}

impl HtmlExt for String {
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

    fn finish(&mut self) -> Vec<u8> {
        encode_zstd_9(&minify_html(self))
    }
}
