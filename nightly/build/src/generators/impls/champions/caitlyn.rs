use super::*;

impl Generator for Caitlyn {
    fn generate(&mut self) -> MayFail {
        // 60% / 70% / 80% AD at levels 1 / 7 / 13
        let headshot = stringify!(match ctx.level {
            ..7 => 0.6,
            7..13 => 0.7,
            13.. => 0.8,
        });

        self.ability(Key::P, [("Headshot", Void)])
            .modify(P(Void), |_| {
                f![((headshot) + 0.8 * CritChance / 100) * AttackDamage]
            })?
            .comment(P(Void), "Headshot bonus physical damage")?
            .ability(
                Key::Q,
                [
                    ("Physical Damage", Max),
                    ("Reduced Damage", Min),
                ],
            )
            .ability(Key::W, [("Headshot Damage Increase", Void)])
            .damage_type(W(Void), Physical)?
            .comment(W(Void), "Bonus Headshot damage against trapped enemies")?
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Physical damage", Void)])
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
