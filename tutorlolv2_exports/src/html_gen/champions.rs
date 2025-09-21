// use crate::{MEGA_BLOCK, Url, export_code::*};
// use tutorlolv2_parser::{highlight_json, prettify_json};

// const CHAMPIONS_CSS: &'static str = include_str!("../../assets/champions.css");

// pub fn generate_champion_html() {
//     for i in 0..CHAMPION_FORMULAS.len() {
//         let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
//         let mut html = String::new();

//         html.push_str(&format!(
//             "<html><head>{}</head><body>",
//             format!(
//                 "
//                 <title>{}</title>
//                 <meta charset=\"utf-8\">
//                 <style>{}</style>
//                 ",
//                 champion_id.as_str(),
//                 CHAMPIONS_CSS
//             )
//         ));

//         html.push_str(&format!(
//             r#"
//             <div class="flex items-center gap-4">
//                 <img src="{}/{:?}.avif" class="w-12 h-12">
//                 <h1>
//                     {}
//                 </h1>
//             </div>
//             "#,
//             Url::CHAMPIONS,
//             champion_id,
//             champion_id.as_str()
//         ));
//         html.push_str(&format!(
//             r#"
//             <div>
//                 <p>Commonly built items per position</p>
//                 <div class="grid grid-cols-2 gap-2">
//                     {}
//                 </div>
//             <div>
//             "#,
//             RECOMMENDED_ITEMS[i]
//                 .iter()
//                 .enumerate()
//                 .map(|(index, recommended_items)| {
//                     let mut result = String::new();
//                     result.push_str(match index {
//                         0 => "<span>Top</span>",
//                         1 => "<span>Jungle</span>",
//                         2 => "<span>Middle</span>",
//                         3 => "<span>ADC</span>",
//                         4 => "<span>Support</span>",
//                         _ => unreachable!(),
//                     });
//                     result.push_str(
//                         &recommended_items
//                             .iter()
//                             .map(|item_id| {
//                                 format!(
//                                     r#"
//                                     <div class="flex items-center gap-3">
//                                         <img class="w-8 h-8" src="{}/{}.avif">
//                                         <span>{}</span>
//                                     </div>
//                                     "#,
//                                     Url::ITEMS,
//                                     item_id.to_riot_id(),
//                                     ITEM_ID_TO_NAME.get(*item_id as usize).unwrap()
//                                 )
//                             })
//                             .collect::<Vec<String>>()
//                             .join(""),
//                     );
//                     result
//                 })
//                 .collect::<Vec<String>>()
//                 .join("")
//         ));

//         let positions = CHAMPION_POSITIONS[i];

//         html.push_str(&format!(
//             "<div>Positions: {}</div>",
//             positions
//                 .iter()
//                 .map(|position| format!("{:?}", position))
//                 .collect::<Vec<String>>()
//                 .join(", ")
//         ));

//         html.push_str(&format!(
//             r#"
//             <div>
//                 <p>This champion has the following internal code:</p>
//                 <code class="text-[#D4D4D4] text-left text-wrap break-all">
//                     {}
//                 </code>
//             </div>
//             "#,
//             {
//                 let offsets = CHAMPION_FORMULAS[i];
//                 MEGA_BLOCK
//                     .get(offsets.0 as usize..offsets.1 as usize)
//                     .unwrap()
//             }
//         ));

//         html.push_str(&format!(
//             r#"
//             <div>
//                 <h2>
//                     This champion's JSON data can be generated using
//                     <a href="https://github.com/LuizGomes56/tutorlolv2">tutorlolv2_dev</a>
//                     crate on Github
//                 </h2>
//                 <code class="text-[#D4D4D4] text-left text-wrap break-all">
//                     {}
//                 </code>
//             </div>
//             "#,
//             {
//                 let offsets = CHAMPION_GENERATOR[i];
//                 MEGA_BLOCK
//                     .get(offsets.0 as usize..offsets.1 as usize)
//                     .unwrap()
//             },
//         ));

//         html.push_str(&format!(
//             r#"
//             <div>
//                 <h2>Example of generated JSON file</h2>
//                 <code class="text-[#D4D4D4] text-left text-wrap break-all">
//                     {}
//                 </code>
//             </div>
//             "#,
//             highlight_json(&prettify_json(
//                 &std::fs::read_to_string(&format!("internal/champions/{:?}.json", champion_id))
//                     .unwrap()
//             )),
//         ));

//         html.push_str("</body></html>");

//         std::fs::write(format!("html/{:?}.html", champion_id), html).unwrap();
//     }
// }

use crate::{MEGA_BLOCK, Url, export_code::*};
use tutorlolv2_parser::{highlight_json, prettify_json};

const CHAMPIONS_CSS: &'static str = include_str!("../../assets/champions.css");

pub fn generate_champion_html() {
    for i in 0..CHAMPION_FORMULAS.len() {
        let champion_id = unsafe { std::mem::transmute::<_, ChampionId>(i as u8) };
        let mut html = String::new();

        html.push_str(&format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - tutorlolv2 docs</title>
    <style>{CHAMPIONS_CSS}</style>
</head>
<body style="margin: 0; padding: 0; box-sizing: border-box; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif; line-height: 1.6; background-color: #121214; color: white; min-height: 100vh;">
    <div style="max-width: 1200px; margin: 0 auto; padding: 20px;">
        <header style="background: #202020; color: white; padding: 30px; border-radius: 15px; margin-bottom: 30px; box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);">
            <div style="display: flex; align-items: center; gap: 20px;">
                <img src="{}/{:?}.avif" alt="{}" style="width: 80px; height: 80px; border-radius: 50%; border: 4px solid rgba(255, 255, 255, 0.3); box-shadow: 0 4px 15px rgba(0, 0, 0, 0.5);">
                <div style="flex: 1;">
                    <h1 style="font-size: 2.5rem; font-weight: 700; margin: 0 0 10px 0; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);">{}</h1>
                    <div style="display: flex; align-items: center; gap: 10px; flex-wrap: wrap;">
                        <span style="font-size: 1rem; opacity: 0.9; font-weight: 500;">Positions:</span>
                        {}
                    </div>
                </div>
            </div>
        </header>

        <main style="display: flex; flex-direction: column; gap: 30px;">"#,
            champion_id.as_str(),
            Url::CHAMPIONS,
            champion_id,
            champion_id.as_str(),
            champion_id.as_str(),
            CHAMPION_POSITIONS[i]
                .iter()
                .map(|position| format!(r#"<span style="background: rgba(255, 255, 255, 0.1); padding: 4px 12px; border-radius: 20px; font-size: 0.85rem; font-weight: 500; border: 1px solid rgba(255, 255, 255, 0.2);">{:?}</span>"#, position))
                .collect::<Vec<String>>()
                .join("")
        ));

        html.push_str(
            r#"
            <section style="background: #1e1e1e; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">Recommended Items by Position</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px;">"#,
        );

        let position_names = ["Top", "Jungle", "Middle", "ADC", "Support"];
        for (index, recommended_items) in RECOMMENDED_ITEMS[i].iter().enumerate() {
            if !recommended_items.is_empty() {
                html.push_str(&format!(
                    r#"
                    <div style="background: #2a2a2a; padding: 20px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3); border: 1px solid #444;">
                        <h3 style="font-size: 1.1rem; font-weight: 600; margin: 0 0 15px 0; color: white; text-align: center; background: #202020; padding: 8px; border-radius: 6px;">{}</h3>
                        <div style="display: flex; flex-direction: column; gap: 10px;">
                            {}
                        </div>
                    </div>"#,
                    position_names.get(index).unwrap_or(&"Unknown"),
                    recommended_items
                        .iter()
                        .map(|item_id| {
                            format!(
                                r#"<div style="display: flex; align-items: center; gap: 12px; padding: 8px; border-radius: 6px; transition: all 0.2s ease; border: 1px solid transparent;" onmouseover="this.style.backgroundColor='#333'; this.style.borderColor='#555'; this.style.transform='translateY(-1px)';" onmouseout="this.style.backgroundColor='transparent'; this.style.borderColor='transparent'; this.style.transform='translateY(0)';">
                                    <img style="width: 32px; height: 32px; border-radius: 4px; box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);" src="{}/{}.avif" alt="{}">
                                    <span style="font-size: 0.9rem; color: #ddd; font-weight: 500;">{}</span>
                                </div>"#,
                                Url::ITEMS,
                                item_id.to_riot_id(),
                                ITEM_ID_TO_NAME.get(*item_id as usize).unwrap_or(&"Unknown"),
                                ITEM_ID_TO_NAME.get(*item_id as usize).unwrap_or(&"Unknown")
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("")
                ));
            }
        }

        html.push_str(
            r#"
                </div>
            </section>"#,
        );

        html.push_str(&format!(
            r#"
            <section style="background: #1a1a1a; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">Champion Internal Code</h2>
                <code>{}</code>
            </section>"#,
            {
                let offsets = CHAMPION_FORMULAS[i];
                MEGA_BLOCK
                    .get(offsets.0 as usize..offsets.1 as usize).unwrap()
            }
        ));

        html.push_str(&format!(
            r#"
            <section style="background: #1a1a1a; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">JSON Data Generator</h2>
                <p style="margin: 0 0 20px 0; font-size: 1rem; line-height: 1.7; color: #ddd;">
                    This champion's JSON data can be generated using the 
                    <a href="https://github.com/LuizGomes56/tutorlolv2" style="color: #90cdf4; text-decoration: underline; font-weight: 600;" target="_blank" rel="noopener noreferrer" onmouseover="this.style.color='#63b3ed';" onmouseout="this.style.color='#90cdf4';">tutorlolv2_dev</a> crate available on Github.
                </p>
                <code>{}</code>
            </section>"#,
            {
                let offsets = CHAMPION_GENERATOR[i];
                MEGA_BLOCK
                    .get(offsets.0 as usize..offsets.1 as usize).unwrap()
            }
        ));

        html.push_str(&format!(
            r#"
            <section style="background: #1a1a1a; padding: 25px; border-radius: 12px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); border: 1px solid #333;">
                <h2 style="font-size: 1.5rem; font-weight: 600; margin: 0 0 20px 0; color: white; border-bottom: 2px solid #333; padding-bottom: 10px;">Generated JSON File Example</h2>
                <code>{}</code>
            </section>"#,
            highlight_json(&prettify_json(
                &std::fs::read_to_string(&format!("internal/champions/{:?}.json", champion_id))
                    .unwrap_or_else(|_| "{}".to_string())
            ))
        ));

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

        std::fs::write(format!("html/{:?}.html", champion_id), html).unwrap();
    }
}
