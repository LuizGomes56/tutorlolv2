use super::*;

impl Generator for Jax {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Additional Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (1, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1),   /* Additional Magic Damage */
                    (5, Void), /* Magic Damage */
                ],
            )
            .damage_type(Q(Void), Physical)?
            .combo([
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Min)),
            ])?
            .combo([
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(E(Min)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
            ])?
            .combo([
                Ability(E(Min)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
            ])?
            .combo([Attack, Ability(Q(Void)), Attack, Ability(W(Void))])?
            .combo([
                Ability(E(Min)),
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(E(Min)),
                Attack,
            ])?
            .end()
    }
}
