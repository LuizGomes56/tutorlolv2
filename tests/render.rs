#![cfg(feature = "yew")]

use strum::VariantArray;
use tutorlolv2::{
    CastId, ChampionId, EntityId, ItemId, RuneId,
    yew::render::{FormulaRenderer, MayFail},
};

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

    std::fs::write("render.html", html)?;

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
