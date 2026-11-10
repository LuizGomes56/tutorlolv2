use super::*;

impl Generator for MissFortune {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![(0.5 + 0.5 / 17 * (Level - 1)) * AttackDamage]
            })?
            .comment(P(Void), "Love Tap bonus damage against unmarked enemies")?
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Maximum Total Physical Damage", Max),
                    ("Physical Damage per Wave", Min),
                ],
            )
            .combos([
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Max)), Ability(R(Max))],
                &[Ability(E(Max)), Attack, Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
