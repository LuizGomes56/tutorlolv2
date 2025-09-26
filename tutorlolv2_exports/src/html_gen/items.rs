use crate::{
    Url,
    export_code::*,
    html::{BASE_CSS, HtmlExt, Source, offset_to_str},
};

pub fn generate_item_html() {
    for i in 0..ITEM_FORMULAS.len() {
        let item_id = unsafe { std::mem::transmute::<_, ItemId>(i as u16) };
        println!("Generating {item_id:#?} html");
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
            Url::ITEMS,
            item_id.to_riot_id(),
            name = ITEM_ID_TO_NAME[item_id as usize],
        ));

        html.code_section("Rust - Internal code", offset_to_str(ITEM_FORMULAS[i]));
        html.code_section(
            "JSON - Intermediate representation",
            &format!("internal/items/{}.json", item_id.to_riot_id()).json_code(),
        );

        html.footer();
        html.finish(Source::Items, item_id);
    }
}
