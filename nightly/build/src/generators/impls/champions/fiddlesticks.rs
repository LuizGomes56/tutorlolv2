use super::*;

impl Generator for Fiddlesticks {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Increased Magic Damage", Max),
            ],
        )
        .modify(Q(Min), |_| {
            f![(0.04 + 0.005 * (QLevel - 1) + 0.0003 * AbilityPower) * EnemyCurrentHealth]
        })?
        .modify(Q(Max), |_| {
            f![(0.08 + 0.01 * (QLevel - 1) + 0.0006 * AbilityPower) * EnemyCurrentHealth]
        })?
        .comment(Q(Max), "Against feared or recently feared enemies")?
        .ability(
            Key::W,
            [
                ("Damage per Instance", Min),
                ("Total Magic Damage", Max),
                ("Last Tick of Damage", _1),
            ],
        )
        .ability(Key::E, [("Magic Damage", Void)])
        .ability(
            Key::R,
            [
                ("Magic Damage per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(R(Max)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Ability(W(Max)),
            ],
            &[Ability(Q(Max)), Ability(W(Max)), Ability(E(Void)), Attack],
            &[
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(Q(Max)),
                Ability(W(Max)),
            ],
            &[Ability(Q(Max)), Ability(E(Void)), Ability(W(Max))],
            &[Ability(E(Void)), Ability(Q(Max)), Ability(W(Max))],
            &[
                Ability(R(Max)),
                Ability(Q(Max)),
                Ability(W(Max)),
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
