use tutorlolv2::{
    CastId, ChampionId, EntityId, ItemId, RuneId,
    yew::render::{MayFail, Renderer},
};

static CSS: &str = include_str!("../style.css");

#[test]
fn pack_repr_build() -> MayFail {
    let mut html = format!("<html><head><style>{CSS}</style></head><body><pre>",);

    html += &Renderer::tower_global();
    html += "\n";
    html += &Renderer::tower_fn();
    html += "\n";

    for champion_id in ChampionId::VALUES {
        for metadata in champion_id.abilities() {
            html += &champion_id.render_fn(metadata.kind)?;
            html += "\n";
        }

        html += &champion_id.render_global()?;
        html += "\n";
    }

    fn render_formulas<T: CastId>(html: &mut String) -> MayFail {
        for value in T::VALUES {
            let (function, global) = match value.entity() {
                EntityId::Item(v) => (v.render_fn()?, v.render_global()?),
                EntityId::Rune(v) => (v.render_fn()?, v.render_global()?),
                _ => unreachable!(),
            };

            html.push_str(&function);
            html.push('\n');
            html.push_str(&global);
            html.push('\n');
        }

        Ok(())
    }

    render_formulas::<ItemId>(&mut html)?;
    render_formulas::<RuneId>(&mut html)?;

    html += "</pre></body></html>";

    std::fs::write("render.html", html)?;

    Ok(())
}
