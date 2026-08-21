use tutorlolv2::{
    CastId, ChampionId, EntityId, ItemId, RuneId,
    yew::render::{FormulaRenderer, MayFail},
};
use tutorlolv2_codec::EntityKind;

static CSS: &str = include_str!("../style.css");

#[test]
fn pack_repr_build() -> MayFail {
    let mut html = format!("<html><head><style>{CSS}</style></head><body><pre>");

    static GEN_BIN: &[u8] = include_bytes!("../generator.bin");
    let gdb = tutorlolv2_codec::generator::GeneratorDb::new(GEN_BIN)?;

    html += &FormulaRenderer::tower_global();
    html += "\n";
    html += &FormulaRenderer::tower_fn();
    html += "\n";

    for champion_id in ChampionId::VALUES {
        html += &gdb
            .render_html(EntityKind::Champion, champion_id.index() as _)?
            .unwrap();
        html += "\n";

        for metadata in champion_id.abilities() {
            html += &champion_id.render_fn(metadata.kind)?;
            html += "\n";
        }

        html += &champion_id.render_global()?;
        html += "\n";
    }

    fn render_formulas<T: CastId>(
        gdb: &tutorlolv2_codec::GeneratorDb<'_>,
        html: &mut String,
    ) -> MayFail {
        for value in T::VALUES {
            let (function, global, genr) = match value.entity() {
                EntityId::Item(v) => (
                    v.render_fn()?,
                    v.render_global()?,
                    gdb.render_html(EntityKind::Item, value.index() as _)?,
                ),
                EntityId::Rune(v) => (
                    v.render_fn()?,
                    v.render_global()?,
                    gdb.render_html(EntityKind::Rune, value.index() as _)?,
                ),
                _ => unreachable!(),
            };

            if let Some(generator) = genr {
                html.push_str(&generator);
                html.push('\n');
            }
            html.push_str(&function);
            html.push('\n');
            html.push_str(&global);
            html.push('\n');
        }

        Ok(())
    }

    render_formulas::<ItemId>(&gdb, &mut html)?;
    render_formulas::<RuneId>(&gdb, &mut html)?;

    html += "</pre></body></html>";

    std::fs::write("render.html", html)?;

    Ok(())
}
