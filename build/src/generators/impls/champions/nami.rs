use super::*;

impl Generator for Nami {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (1, Max), /* Magic Damage */
                    (2, Min), /* Minimum Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Min), /* Bonus Magic Damage Per Hit */
                    (2, Max), /* Total Bonus Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(W(Max)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(E(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Ability(W(Max)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(E(Max)),
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(W(Max)), Ability(E(Max)), Attack])?
            .combo([Ability(E(Max)), Ability(W(Max)), Attack])?
            .combo([
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Ability(W(Max)),
                Attack,
                Attack,
            ])?
            .end()
    }
}
