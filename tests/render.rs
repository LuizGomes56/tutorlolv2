#![cfg(feature = "formula-docs")]

use strum::VariantArray;
use tutorlolv2::{
    CastId, ChampionId, EntityId, ItemId, RuneId,
    yew::render::{FormulaRenderer, MayFail},
};
use tutorlolv2_codec::{Class, Spans};

static CSS: &str = include_str!("../style.css");

#[test]
fn pack_repr_build() -> MayFail {
    let mut html = format!("<html><head><style>{CSS}</style></head><body><pre>");

    for builtin in [
        FormulaRenderer::ignite_fn(),
        FormulaRenderer::onhit_effect_global(),
        FormulaRenderer::onhit_effect_fn(),
        FormulaRenderer::critical_strike_global(),
        FormulaRenderer::critical_strike_fn(),
        FormulaRenderer::basic_attack_global(),
        FormulaRenderer::basic_attack_fn(),
    ] {
        html.push_str(&builtin);
        html.push_str("\n\n");
    }

    html += &FormulaRenderer::tower_global();
    html += "\n";
    html += &FormulaRenderer::tower_fn();
    html += "\n";

    for champion_id in ChampionId::VARIANTS {
        html += &champion_id.render_generator().unwrap();
        html += "\n";

        for metadata in champion_id.abilities() {
            html += &champion_id.render_fn(metadata.kind)?;
            html += "\n";
        }

        html += &champion_id.render_global()?;
        html += "\n";
    }

    fn render_formulas<T: CastId + VariantArray>(html: &mut String) -> MayFail {
        for value in T::VARIANTS {
            let (function, global) = match value.entity() {
                EntityId::Item(v) => (v.render_fn()?, v.render_global()?),
                EntityId::Rune(v) => (v.render_fn()?, v.render_global()?),
                _ => unreachable!(),
            };

            if let Some(generator) = value.render_generator() {
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

    render_formulas::<ItemId>(&mut html)?;
    render_formulas::<RuneId>(&mut html)?;

    html += "</pre></body></html>";

    std::fs::create_dir_all("html")?;
    std::fs::write("html/render.html", html)?;

    Ok(())
}

#[test]
fn builtin_comments_are_highlighted_without_delimiters() {
    let comment = tutorlolv2_codec::Highlighter::span(
        tutorlolv2_codec::Class::Comment,
        "* physical_multiplier",
    );
    for html in [
        FormulaRenderer::critical_strike_fn(),
        FormulaRenderer::basic_attack_global(),
        FormulaRenderer::basic_attack_fn(),
    ] {
        assert!(html.contains(&comment));
        assert!(!html.contains("/*"));
        assert!(!html.contains("*/"));
    }
}

#[test]
fn formulas_changed_by_game_modes_render_their_map_branches() -> MayFail {
    let mut branches = Vec::new();

    for champion_id in ChampionId::VARIANTS {
        for metadata in champion_id.abilities() {
            let rendered = champion_id.render_fn_markup::<Spans>(metadata.kind)?;

            if rendered.text.contains("#[cold]") {
                branches.push(rendered);
            }
        }
    }

    for item_id in ItemId::VARIANTS {
        if let EntityId::Item(item) = item_id.entity() {
            let rendered = item.render_fn_markup::<Spans>()?;

            if rendered.text.contains("#[cold]") {
                branches.push(rendered);
            }
        }
    }

    assert!(!branches.is_empty(), "No formula renders a map branch");

    for rendered in &branches {
        let text = &rendered.text;
        assert!(text.contains("match"), "{text}");
        assert!(text.contains("Aram") || text.contains("Arena"), "{text}");
        assert!(!text.contains("cold_path"), "{text}");
        assert!(!text.contains("tutorlolv2_types"), "{text}");

        for (offset, _) in text.match_indices("#[cold]") {
            let cold = offset + 2..offset + 6;
            assert!(
                rendered.runs().any(|(range, class)| {
                    class == Some(Class::Function)
                        && range.start <= cold.start
                        && range.end >= cold.end
                }),
                "cold attribute must use function highlighting: {text}"
            );
        }
    }

    Ok(())
}
