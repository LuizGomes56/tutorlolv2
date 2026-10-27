use super::*;

impl Generator for Syndra {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Void), /* Bonus Damage */
                    (1, Min),  /* Magic Damage */
                    (2, Max),  /* Total Mixed Damage */
                ],
            );

        let w_bonus = self.damage_of(W(Void))?.clone();

        self.modify(W(Max), |dmg: &str| f![((dmg) * MagicMultiplier) + w_bonus])?
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Void), /* Magic Damage per Sphere */
                    (2, Max),  /* Maximum Magic Damage */
                    (3, Min),  /* Minimum Magic Damage */
                ],
            )
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
            ])?
            .combo([Ability(Q(Void)), Ability(R(Void)), Ability(E(Void))])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
                Attack,
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(W(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Void)),
            ])?
            .end()
    }
}
