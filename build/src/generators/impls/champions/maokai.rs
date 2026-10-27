use super::*;

impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (4, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .combo([
                Ability(R(Void)),
                Ability(W(Void)),
                Ability(E(Max)),
                Ability(Q(Void)),
                Attack,
                Attack,
            ])?
            .combo([Ability(W(Void)), Ability(Q(Void)), Ability(R(Void)), Attack])?
            .combo([Ability(Q(Void)), Ability(W(Void)), Attack])?
            .combo([
                Ability(W(Void)),
                Ability(E(Max)),
                Ability(Q(Void)),
                Ability(R(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Max)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Max)),
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Ability(Q(Void)), Attack])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Ability(E(Max)),
                Attack,
                Attack,
            ])?
            .end()
    }
}
