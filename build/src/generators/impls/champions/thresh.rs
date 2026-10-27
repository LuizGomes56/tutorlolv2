use super::*;

impl Generator for Thresh {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void),  /* Magic Damage */
                    (1, _1Max), /* Maximum Bonus Magic Damage */
                    (2, _1Min), /* Minimum Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(Q(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void)), Attack])?
            .combo([Ability(Q(Void)), Ability(R(Void)), Ability(E(Void)), Attack])?
            .combo([Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack])?
            .combo([Ability(R(Void)), Ability(E(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(E(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Attack])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Attack, Ability(E(Void)), Attack])?
            .end()
    }
}
