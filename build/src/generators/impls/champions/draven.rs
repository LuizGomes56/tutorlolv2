use super::*;

impl Generator for Draven {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(
                Key::R,
                [
                    (0, _1Min), /* Minimum Physical Damage */
                    (1, Min),   /* Minimum Total Damage */
                    (2, _1Max), /* Physical Damage */
                    (3, Max),   /* Total Physical Damage */
                ],
            )
            .combo([
                Ability(Q(Void)),
                Attack,
                Attack,
                Ability(E(Void)),
                Ability(R(Max)),
                Attack,
            ])?
            .combo([Attack, Ability(Q(Void)), Attack, Attack, Attack])?
            .combo([
                Ability(R(Max)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
            ])?
            .combo([Ability(E(Void)), Ability(Q(Void)), Attack])?
            .combo([Attack, Ability(Q(Void))])?
            .combo([
                Ability(Q(Void)),
                Ability(R(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Attack, Ability(E(Void)), Attack, Attack, Attack])?
            .end()
    }
}
