use crate::{
    MEGA_BLOCK, Url,
    export_code::*,
    html::{BASE_CSS, HtmlExt},
};

pub fn generate_rune_html() {
    for i in 0..RUNE_FORMULAS.len() {
        let rune_id = unsafe { std::mem::transmute::<_, RuneId>(i as u8) };
        println!("Generating {rune_id:#?} html");
        let mut html = String::new();

        html.push_str(&format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name} - tutorlolv2 docs</title>
    <style>{BASE_CSS}</style>
</head>
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
            name = RUNE_ID_TO_NAME[rune_id as usize],
        ));

        html.code_section("Rune Internal Code", {
            let offsets = RUNE_FORMULAS[i];
            MEGA_BLOCK
                .get(offsets.0 as usize..offsets.1 as usize)
                .unwrap()
        });

        html.push_str(
            r#"
        </main>

        <footer style="text-align: center; padding: 20px; margin-top: 40px; color: #888; font-size: 0.9rem; border-top: 1px solid #333;">
            <p style="margin: 0;">Documentation automatically generated - tutorlolv2</p>
        </footer>
    </div>
</body>
</html>"#,
        );

        std::fs::write(format!("html/runes/{:?}.zst", rune_id), html.finish()).unwrap();
    }
}
