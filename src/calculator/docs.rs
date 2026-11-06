//! Documentation of damage cells and headers: the web app's `hover.rs`.
//!
//! The web encodes a [`Docs`] value and the context values into HTML attributes, and
//! `hover.js` renders them when the cell is hovered. Here the same values are captured by the
//! tooltip builder.
use crate::components::code::Highlighted;
use tutorlolv2::{
    AbilityId, CastId, ChampionId, Ctx, CtxVar, EntityId, ItemId, RuneId,
    formula_docs::FormulaRenderer,
};
use tutorlolv2_codec::{Class, Markup, Spans};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Docs {
    Champion(ChampionId, AbilityId),
    /// Min and max abilities that are displayed together in a single cell.
    ChampionMerged(ChampionId, AbilityId, AbilityId),
    Item(ItemId),
    Rune(RuneId),
    Global(EntityId),
    Builtin(Builtin, DocView),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Builtin {
    Tower,
    BasicAttack,
    CriticalStrike,
    Onhit,
    Ignite,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocView {
    Global,
    Function,
}

impl Docs {
    pub fn global(id: impl CastId) -> Self {
        Self::Global(id.entity())
    }

    fn global_markup(id: EntityId) -> Highlighted {
        match id {
            EntityId::Champion(id) => id.render_global_markup::<Spans>(),
            EntityId::Item(id) => id.render_global_markup::<Spans>(),
            EntityId::Rune(id) => id.render_global_markup::<Spans>(),
        }
        .into()
    }

    fn builtin(builtin: Builtin, view: DocView) -> Spans {
        match (builtin, view) {
            (Builtin::Tower, DocView::Function) => FormulaRenderer::tower_fn_markup(),
            (Builtin::Tower, DocView::Global) => FormulaRenderer::tower_global_markup(),
            (Builtin::Onhit, DocView::Function) => FormulaRenderer::onhit_effect_fn_markup(),
            (Builtin::Onhit, DocView::Global) => FormulaRenderer::onhit_effect_global_markup(),
            (Builtin::CriticalStrike, DocView::Function) => {
                FormulaRenderer::critical_strike_fn_markup()
            }
            (Builtin::CriticalStrike, DocView::Global) => {
                FormulaRenderer::critical_strike_global_markup()
            }
            (Builtin::BasicAttack, DocView::Function) => FormulaRenderer::basic_attack_fn_markup(),
            (Builtin::BasicAttack, DocView::Global) => {
                FormulaRenderer::basic_attack_global_markup()
            }
            // Ignite has only its function, which the web shows for both views.
            (Builtin::Ignite, _) => FormulaRenderer::ignite_fn_markup(),
        }
    }

    /// One block per documented function, like the web's `<pre>` elements.
    pub fn blocks(self) -> Vec<Highlighted> {
        match self {
            Self::Champion(champion, ability) => {
                vec![champion.render_fn_markup::<Spans>(ability).into()]
            }
            Self::ChampionMerged(champion, min, max) => vec![
                champion.render_fn_markup::<Spans>(min).into(),
                champion.render_fn_markup::<Spans>(max).into(),
            ],
            Self::Item(item) => vec![item.render_fn_markup::<Spans>().into()],
            Self::Rune(rune) => vec![rune.render_fn_markup::<Spans>().into()],
            Self::Global(id) => vec![Self::global_markup(id)],
            Self::Builtin(builtin, view) => vec![Highlighted::new(&Self::builtin(builtin, view))],
        }
    }
}

/// Numeric value shown in identifier tooltips (`hover::ident_display_value`).
fn display_value(ident: CtxVar, value: f32) -> f32 {
    match ident {
        CtxVar::AttackSpeed
        | CtxVar::BaseAttackSpeed
        | CtxVar::BonusAttackSpeed
        | CtxVar::PhysicalMultiplier
        | CtxVar::MagicMultiplier
        | CtxVar::SteelcapsEffect
        | CtxVar::ArmorPenetrationPercent
        | CtxVar::MagicPenetrationPercent
        | CtxVar::MissingHealthRatio
        | CtxVar::RanduinEffect
        | CtxVar::RocksolidEffect => (value * 100.0).round() / 100.0 + 0.0,
        // Adding zero normalizes rounded negative zero for display.
        _ => value.round() + 0.0,
    }
}

/// The `name = value` lines above the documentation, one per context variable.
fn idents_block(idents: &[CtxVar], ctx: &Ctx) -> Option<Highlighted> {
    if idents.is_empty() {
        return None;
    }

    let mut spans = Spans::default();

    for (i, &ident) in idents.iter().enumerate() {
        if i > 0 {
            spans.push(None, "\n");
        }

        // `<span class="C12">name</span> = <span class="C8">value</span>`
        spans.push(Some(Class::Variable), ident.to_snake_case());
        spans.push(None, " = ");
        spans.push(
            Some(Class::Number),
            &display_value(ident, ctx.get(ident)).to_string(),
        );
    }

    Some(Highlighted::new(&spans))
}

/// Tooltip blocks for a cell: its context values, then its documentation.
pub fn cell_blocks(idents: &[CtxVar], ctx: &Ctx, docs: Docs) -> Vec<Highlighted> {
    idents_block(idents, ctx)
        .into_iter()
        .chain(docs.blocks())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::display_value;
    use tutorlolv2::CtxVar;

    /// `displayed_ident_value_matches_zero_filter_and_rounding` from the web.
    #[test]
    fn displayed_values_round_like_the_web() {
        for value in [0.0, 0.0000038146973, -0.000061035156, 0.49, -0.49] {
            assert_eq!(display_value(CtxVar::BonusMagicResist, value), 0.0);
        }

        for (value, expected) in [(0.5, 1.0), (-0.5, -1.0), (778.69995, 779.0)] {
            assert_eq!(display_value(CtxVar::AbilityPower, value), expected);
        }

        for ident in [CtxVar::AttackSpeed, CtxVar::BonusAttackSpeed] {
            assert_eq!(display_value(ident, 0.0001), 0.0);
            assert_eq!(display_value(ident, 1.44575), 1.45);
            assert_eq!(display_value(ident, -0.42209995), -0.42);
        }
    }
}
