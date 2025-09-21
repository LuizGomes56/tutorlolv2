mod export_code;

use bincode::{Decode, Encode};
use export_code::*;
use tutorlolv2_parser::highlight;
use tutorlolv2_shared::*;

pub static MEGA_BLOCK: &'static str = include_str!("../mega_block.txt");

pub fn generate_champion_html() {
    for i in 0..CHAMPION_FORMULAS.len() {
        let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
        let mut html = String::new();

        html.push_str(&format!(
            "<html><head>{}</head><body>",
            format!(
                "<title>{}</title><meta charset=\"utf-8\">",
                champion_id.as_str()
            )
        ));

        html.push_str(&format!("<h1>{}</h1>", champion_id.as_str()));
        html.push_str(&format!(
            r#"
            <div>
                <p>Commonly built items per position</p>
                <div class="grid grid-cols-2 gap-2">
                    {}
                </div>
            <div>
            "#,
            RECOMMENDED_ITEMS[i]
                .iter()
                .enumerate()
                .map(|(index, recommended_items)| {
                    let mut result = String::new();
                    result.push_str(match index {
                        0 => "<span>Top</span>",
                        1 => "<span>Jungle</span>",
                        2 => "<span>Middle</span>",
                        3 => "<span>ADC</span>",
                        4 => "<span>Support</span>",
                        _ => unreachable!(),
                    });
                    result.push_str(
                        &recommended_items
                            .iter()
                            .map(|item_id| {
                                format!(
                                    r#"
                                    <div class="flex items-center gap-3">
                                        <img class="w-8 h-8" src="items/{}.avif">
                                        <span>{}</span>
                                    </div>
                                    "#,
                                    item_id.to_riot_id(),
                                    ITEM_ID_TO_NAME.get(*item_id as usize).unwrap()
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(""),
                    );
                    result
                })
                .collect::<Vec<String>>()
                .join("")
        ));
        html.push_str(&format!(
            r#"
            <div>
                <p>This champion has the following internal code:</p>
                <code class="text-[#D4D4D4] text-left text-wrap break-all">
                    {}
                </code>
            </div>
            "#,
            {
                let offsets = CHAMPION_FORMULAS[i];
                MEGA_BLOCK
                    .get(offsets.0 as usize..offsets.1 as usize)
                    .unwrap()
            }
        ));

        let positions = CHAMPION_POSITIONS[i];

        html.push_str(&format!(
            "<div>Positions: {}</div>",
            positions
                .iter()
                .map(|position| format!("{:?}", position))
                .collect::<Vec<String>>()
                .join(", ")
        ));

        html.push_str(&format!(
            r#"
            <div>
                <h2>
                    <span>This champion's JSON data can be generated using </span>
                    <span class="font-bold">tutorlolv2_dev</span>
                    <span> crate on Github</span> 
                </h2>
                <code class="text-[#D4D4D4] text-left text-wrap break-all">
                    {}
                </code>
            </div>
            "#,
            {
                let offsets = CHAMPION_GENERATOR[i];
                MEGA_BLOCK
                    .get(offsets.0 as usize..offsets.1 as usize)
                    .unwrap()
            },
        ));

        html.push_str(&format!(
            r#"
            <div>
                <h2>Example of generated JSON file</h2>
                <code class="text-[#D4D4D4] text-left text-wrap break-all">
                    {}
                </code>
            </div>
            "#,
            highlight(
                &std::fs::read_to_string(&format!("internal/champions/{:?}.json", champion_id))
                    .unwrap()
            ),
        ));

        html.push_str("</body></html>");

        std::fs::write(format!("html/{:?}.html", champion_id), html).unwrap();
    }
}
