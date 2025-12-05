use crate::{
    Url,
    export_code::*,
    html::{HtmlExt, Source, offset_to_str},
};
use std::fmt::Debug;

trait EnumImport {
    fn to_riot_id(&self) -> u32;
    fn cast_usize(&self) -> usize;
}

impl EnumImport for ItemId {
    fn to_riot_id(&self) -> u32 {
        self.to_riot_id()
    }
    fn cast_usize(&self) -> usize {
        *self as usize
    }
}

impl EnumImport for RuneId {
    fn to_riot_id(&self) -> u32 {
        self.to_riot_id()
    }
    fn cast_usize(&self) -> usize {
        *self as usize
    }
}

fn push_recommendation<
    T: 'static + EnumImport + Debug,
    const A: usize,
    const B: usize,
    const C: usize,
>(
    const_iterator: [[&[T]; A]; B],
    html: &mut String,
    url_kind: &'static str,
    const_id_to_name: [&str; C],
    i: usize,
) {
    let position_names = ["Top", "Jungle", "Middle", "ADC", "Support"];

    for (index, static_const) in const_iterator[i].iter().enumerate() {
        if !static_const.is_empty() {
            html.push_str(&format!(
                r#"
                <div style="background: #2a2a2a; padding: 20px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3); border: 1px solid #444;">
                    <h3 style="font-size: 1.1rem; font-weight: 600; margin: 0 0 15px 0; color: white; text-align: center; background: #202020; padding: 8px; border-radius: 6px;">{}</h3>
                    <div style="display: flex; flex-direction: column; gap: 10px;">
                        {}
                    </div>
                </div>"#,
                position_names.get(index).unwrap_or(&"Unknown"),
                static_const
                    .iter()
                    .map(|static_id| {
                        format!(
                            r#"<a href="{}/docs/items/{:?}" style="text-decoration: none; display: flex; cursor: pointer; align-items: center; gap: 12px; padding: 8px; border-radius: 6px; transition: all 0.2s ease; border: 1px solid transparent;" onmouseover="this.style.backgroundColor='#333'; this.style.borderColor='#555'; this.style.transform='translateY(-1px)';" onmouseout="this.style.backgroundColor='transparent'; this.style.borderColor='transparent'; this.style.transform='translateY(0)';">
                                <img style="width: 32px; height: 32px; border-radius: 4px; box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);" src="{}/{}.avif" alt="{}">
                                <span style="font-size: 0.9rem; color: #ddd; font-weight: 500;">{}</span>
                            </a>"#,
                            Url::BASE,
                            static_id,
                            url_kind,
                            static_id.to_riot_id(),
                            const_id_to_name.get(static_id.cast_usize()).unwrap_or(&"Unknown"),
                            const_id_to_name.get(static_id.cast_usize()).unwrap_or(&"Unknown"),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("")
            ));
        }
    }
}

pub fn generate_champion_html() {
    for i in 0..CHAMPION_FORMULAS.len() {
        let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
        println!("Generating {champion_id:#?} html");
        let mut html = String::new();

        let name = CHAMPION_ID_TO_NAME[champion_id as usize];
        html.header(name);
        html.push_str(&format!(
            r#"
<body style="margin: 0; padding: 0; box-sizing: border-box; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif; line-height: 1.6; background-color: #121214; color: white; min-height: 100vh;">
    <div style="max-width: 1200px; margin: 0 auto; padding: 20px;">
        <header style="background: #202020; color: white; padding: 30px; border-radius: 15px; margin-bottom: 30px; box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);">
            <div style="display: flex; align-items: center; gap: 20px;">
                <img src="{}/{:?}.avif" alt="{name}" style="width: 80px; height: 80px; border-radius: 50%; border: 4px solid rgba(255, 255, 255, 0.3); box-shadow: 0 4px 15px rgba(0, 0, 0, 0.5);">
                <div style="flex: 1;">
                    <h1 style="font-size: 2.5rem; font-weight: 700; margin: 0 0 10px 0; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);">{name}</h1>
                    <div style="display: flex; align-items: center; gap: 10px; flex-wrap: wrap;">
                        <span style="font-size: 1rem; opacity: 0.9; font-weight: 500;">Positions:</span>
                        {}
                    </div>
                </div>
            </div>
        </header>

        <main style="display: flex; flex-direction: column; gap: 30px;">"#,
            Url::CHAMPIONS,
            champion_id,
            CHAMPION_POSITIONS[i]
                .iter()
                .map(|position| format!(r#"<span style="background: rgba(255, 255, 255, 0.1); padding: 4px 12px; border-radius: 20px; font-size: 0.85rem; font-weight: 500; border: 1px solid rgba(255, 255, 255, 0.2);">{:?}</span>"#, position))
                .collect::<Vec<String>>()
                .join(""),
        ));

        html.push_str(
            r#"
            <section style="background: #1e1e1e; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">Recommended Items by Position</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px;">"#,
        );
        push_recommendation(RECOMMENDED_ITEMS, &mut html, Url::ITEMS, ITEM_ID_TO_NAME, i);
        html.push_str(
            r#"
                </div>
            </section>"#,
        );

        html.push_str(
            r#"
            <section style="background: #1e1e1e; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">Recommended Runes by Position</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px;">"#,
        );
        push_recommendation(RECOMMENDED_RUNES, &mut html, Url::RUNES, RUNE_ID_TO_NAME, i);
        html.push_str(
            r#"
                </div>
            </section>"#,
        );
        html.code_section("Rust - Internal code", offset_to_str(CHAMPION_FORMULAS[i]));

        for (ability_like, offsets) in CHAMPION_ABILITIES[i] {
            html.code_section(
                &format!("{ability_like:?}").replace("_", " "),
                offset_to_str(*offsets),
            );
        }

        html.code_section("Rust - Generator", offset_to_str(CHAMPION_GENERATOR[i]));
        html.code_section(
            "JSON - Intermediate representation",
            &format!("internal/champions/{:?}.json", champion_id).json_code(),
        );

        html.footer();
        html.finish(Source::Champions, champion_id);
    }
}
