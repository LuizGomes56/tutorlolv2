use super::*;

impl Generator for Samira {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Description 3", _1Min),
                ("Innate", Min),
            ],
        )
        .clone_with(P(Min), P(Max), |dmg| f![(dmg) * 2])?;

        let p = self.scaling(Key::P, "Description 3")?[0]
            .clone()
            .set_ctx_var(AttackDamage)
            .render(Level)?;

        self.set_damage(P(_1Min), p)?
            .comment(P(_1Min), "Passive against immobilized targets")?
            .clone_with(P(_1Min), P(_1Max), |dmg| f![(dmg) * 6])?
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Physical Damage per Hit", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Physical Damage Per Shot", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .combos([
                &[
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max)), Attack],
            ])?
            .end()
    }
}
