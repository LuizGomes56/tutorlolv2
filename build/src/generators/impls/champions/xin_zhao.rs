use super::*;

impl Generator for XinZhao {
    fn generate(&mut self) -> MayFail {
        // 15% / 30% / 45% / 60% AD (+ 5% / 10% / 15% / 20% AP) at levels 1 / 6 / 11 / 16
        let tier = stringify!(match ctx.level {
            ..6 => 1,
            6..11 => 2,
            11..16 => 3,
            16.. => 4,
        });

        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![(tier) * (0.15 * AttackDamage + 0.05 * AbilityPower)]
            })?
            .comment(P(Void), "Determination third stack bonus damage")?
            .ability(
                Key::Q,
                [
                    ("Bonus Physical Damage", Min),
                    ("Total Bonus Physical Damage", Max),
                ],
            )
            .ability(
                Key::W,
                [
                    ("Slash Total Physical Damage", Min),
                    ("Total Physical Damage", Max),
                    ("Thrust Physical Damage", _1),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Physical Damage", Void)])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(R(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
