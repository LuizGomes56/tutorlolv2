use tutorlolv2::{
    CastId, ChampionId, EntityId, ItemId, RuneId,
    yew::render::{MayFail, Renderer},
};

static PACKED: &[u8] = include_bytes!("../packer.bin");
static CSS: &str = include_str!("../style.css");

#[test]
fn pack_repr_build() -> MayFail {
    let mut html = format!("<html><head><style>{CSS}</style></head><body><pre>",);
    let mut renderer = Renderer::parse(PACKED)?;

    for champion_id in ChampionId::VALUES {
        for metadata in champion_id.abilities() {
            html += &renderer.champion_function(champion_id, metadata.kind)?;
            html += "\n";
        }

        html += &renderer.champion_global(champion_id)?;
    }

    fn render_formulas<T: CastId>(html: &mut String, renderer: &mut Renderer<'_>) -> MayFail {
        for value in T::VALUES {
            let (function, global) = match value.entity() {
                EntityId::Item(v) => (renderer.item_function(v)?, renderer.item_global(v)?),
                EntityId::Rune(v) => (renderer.rune_function(v)?, renderer.rune_global(v)?),
                _ => unreachable!(),
            };

            html.push_str(&function);
            html.push_str(&global);
        }

        Ok(())
    }

    render_formulas::<ItemId>(&mut html, &mut renderer)?;
    render_formulas::<RuneId>(&mut html, &mut renderer)?;

    html += "</pre></body></html>";

    std::fs::write("render.html", html)?;

    Ok(())
}
