use crate::{
    Url,
    export_code::*,
    html::{HtmlExt, Source, offset_to_str},
};
use tutorlolv2_gen::RuneId as RuneId2;

pub fn generate_rune_html() {
    for i in 0..RUNE_FORMULAS.len() {
        let rune_id = unsafe { RuneId2::from_u8_unchecked(i as _) };
        println!("Generating {rune_id:#?} html");
        let mut html = String::new();

        let name = RUNE_ID_TO_NAME[rune_id as usize];
        html.header(name);
        html.push_str(&format!(
            r#"
<body style="margin: 0; padding: 0; box-sizing: border-box; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif; line-height: 1.6; background-color: #121214; color: white; min-height: 100vh;">
    <div style="max-width: 1200px; margin: 0 auto; padding: 20px;">
        <header style="background: #202020; color: white; padding: 30px; border-radius: 15px; margin-bottom: 30px; box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);">
            <div style="display: flex; align-items: center; gap: 20px;">
                <img src="{}/{}.avif" alt="{name}" style="width: 80px; height: 80px; border-radius: 50%; border: 4px solid rgba(255, 255, 255, 0.3); box-shadow: 0 4px 15px rgba(0, 0, 0, 0.5);">
                <h1 style="font-size: 2.5rem; font-weight: 700; margin: 0 0 10px 0; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);">{name}</h1>
            </div>    
        </header>

        <main style="display: flex; flex-direction: column; gap: 30px;">"#,
            Url::RUNES,
            rune_id.to_riot_id(),
        ));

        html.code_section("Rust - Internal code", offset_to_str(RUNE_FORMULAS[i]));
        html.footer();
        html.finish(Source::Runes, rune_id);
    }
}
