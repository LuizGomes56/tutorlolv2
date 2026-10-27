use super::*;

impl Generator for Alistar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .combo([
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Ability(E(Max)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Ability(Q(Void))])?
            .combo([Ability(Q(Void)), Ability(W(Void))])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Ability(W(Void)),
                Ability(E(Max)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Void)), Attack])?
            .end()
    }
}
