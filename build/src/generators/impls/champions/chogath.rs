use super::*;

impl Generator for Chogath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic damage */])
            .ability(Key::W, [(0, Void) /* Magic damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(3, Void) /* Champion True Damage */])
            .combo([
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([Ability(Q(Void)), Ability(W(Void)), Ability(R(Void))])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([
                Ability(W(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Ability(R(Void)),
            ])?
            .combo([
                Ability(W(Void)),
                Attack,
                Ability(E(Max)),
                Ability(Q(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Ability(W(Void)),
                Attack,
                Attack,
            ])?
            .end()
    }
}
