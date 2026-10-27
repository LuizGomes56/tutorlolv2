use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Bonus Physical Damage */])
            .ability(
                Key::W,
                [
                    (2, Max), /* Increased Damage */
                    (4, Min), /* Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Max), /* Increased Damage */
                    (1, Min), /* Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max),  /* Increased Modified Damage */
                    (1, Void), /* Magic Damage */
                    (2, Min),  /* Modified Magic Damage */
                ],
            )
            .damage_type(Q(Void), Physical)?
            .combo([
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
                Attack,
                Ability(E(Min)),
            ])?
            .combo([
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
                Attack,
                Ability(E(Min)),
            ])?
            .combo([
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(R(Void)),
                Attack,
                Attack,
                Ability(E(Min)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(R(Void)),
                Ability(E(Min)),
                Attack,
                Attack,
                Attack,
                Ability(E(Min)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(E(Min)),
                Attack,
                Attack,
                Attack,
                Ability(E(Min)),
            ])?
            .combo([Ability(E(Min)), Ability(Q(Void)), Ability(E(Min)), Attack])?
            .end()
    }
}
