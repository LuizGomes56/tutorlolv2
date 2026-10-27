use super::*;

impl Generator for MissFortune {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(0.5 + 0.5 / 17 * (Level - 1)) * AttackDamage]
            })?
            .comment(P(Void), "Love Tap bonus damage against unmarked enemies")?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max), /* Maximum Total Physical Damage */
                    (1, Min), /* Physical Damage per Wave */
                ],
            )
            .combo([
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Max)),
                Attack,
            ])?
            .combo([
                Attack,
                Attack,
                Ability(E(Max)),
                Ability(Q(Void)),
                Ability(R(Max)),
            ])?
            .combo([
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Ability(R(Max)),
            ])?
            .combo([Ability(E(Max)), Ability(R(Max))])?
            .combo([Ability(E(Max)), Attack, Ability(Q(Void)), Attack])?
            .combo([Ability(Q(Void)), Attack])?
            .end()
    }
}
