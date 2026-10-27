use super::*;

impl Generator for Vex {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 2 */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .combo([
                Ability(R(Min)),
                Ability(R(Min)),
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Min)),
                Ability(R(Min)),
                Ability(W(Void)),
            ])?
            .combo([
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Min)),
                Ability(Q(Void)),
                Ability(R(Min)),
                Attack,
            ])?
            .combo([
                Ability(R(Min)),
                Ability(R(Min)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(R(Min)),
                Ability(Q(Void)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
            ])?
            .combo([Attack, Ability(E(Void)), Attack, Ability(Q(Void)), Attack])?
            .combo([
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .end()
    }
}
