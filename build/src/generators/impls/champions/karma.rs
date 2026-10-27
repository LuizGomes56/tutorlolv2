use super::*;

impl Generator for Karma {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::Q,
                [
                    (4, _1), /* Total Magic Damage */
                    (3, _2), /* Total Damage */
                ],
            )
            .comment(Q(_1), "Mantra empowered impact")?
            .comment(Q(_2), "Mantra empowered impact and Soulflare")?
            .ability_nth(
                1,
                Key::W,
                [
                    (0, Min), /* Magic Damage */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .combo([Ability(Q(Void)), Ability(W(Max)), Attack, Attack, Attack])?
            .combo([
                Ability(W(Max)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
            ])?
            .combo([Ability(W(Max)), Attack, Ability(Q(Void)), Attack])?
            .combo([Ability(W(Max)), Ability(Q(Void))])?
            .combo([Ability(Q(Void)), Ability(W(Max)), Attack, Attack])?
            .combo([Attack, Ability(Q(Void)), Attack])?
            .end()
    }
}
