use super::*;

impl Generator for KogMaw {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(1, Void) /* Magic Damage */])
            .ability(Key::W, [(1, Void) /* Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (1, Min), /* Minimum Magic Damage */
                ],
            )
            .combo([
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Min)),
                Attack,
                Attack,
                Attack,
                Ability(R(Min)),
            ])?
            .combo([Ability(E(Void)), Ability(Q(Void)), Ability(R(Max))])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Max)),
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .end()
    }
}
