use super::*;

impl Generator for Ziggs {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void), /* Magic Damage per Mine */
                    (1, Max),  /* Maximum Total Magic Damage */
                    (2, Min),  /* Reduced Damage per Mine */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max), /* Epicenter Magic Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Max)),
                Attack,
                Ability(W(Void)),
                Ability(W(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(W(Void)),
                Ability(R(Max)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Max)),
            ])?
            .combo([
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Ability(W(Void)),
            ])?
            .combo([Ability(W(Void)), Ability(W(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(E(Void)), Ability(Q(Void)), Attack])?
            .combo([
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(W(Void)),
            ])?
            .end()
    }
}
