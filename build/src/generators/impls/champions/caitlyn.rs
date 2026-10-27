use super::*;

impl Generator for Caitlyn {
    fn generate(&mut self) -> MayFail {
        // 60% / 70% / 80% AD at levels 1 / 7 / 13
        let headshot = stringify!(match ctx.level {
            ..7 => 0.6,
            7..13 => 0.7,
            13.. => 0.8,
        });

        self.ability(Key::P, [(1, Void) /* Headshot */])
            .modify(P(Void), |_| {
                f![((headshot) + 0.8 * CritChance / 100) * AttackDamage]
            })?
            .comment(P(Void), "Headshot bonus physical damage")?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Physical Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Headshot Damage Increase */])
            .damage_type(W(Void), Physical)?
            .comment(W(Void), "Bonus Headshot damage against trapped enemies")?
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Physical damage */])
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
