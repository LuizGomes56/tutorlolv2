use super::*;

impl Generator for Riven {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![(30 + 15 / 17 * (Level - 1)) / 100 * AttackDamage]
            })?
            .ability(
                Key::Q,
                [
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .ability(Key::W, [("Physical Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Maximum Physical Damage", Max),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .damage_types([P(Void), Q(Min), Q(Max), W(Void), R(Min), R(Max)], Physical)?
            .combos([
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(R(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Min)),
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Max))],
                &[Ability(R(Max)), Ability(Q(Max))],
                &[Ability(R(Max)), Ability(W(Void))],
            ])?
            .end()
    }
}
