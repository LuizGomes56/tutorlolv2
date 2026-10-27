use super::*;

impl Generator for Nunu {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Champion Magic Damage */])
            .damage_type(Q(Void), Magic)?
            .ability(
                Key::W,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (2, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, _1),  /* Magic Damage (Root) */
                    (1, Min), /* Magic Damage Per Hit */
                    (2, Max), /* Maximum Total Magic Damage */
                    (4, _2),  /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Maximum Magic Damage */])
            .combo([
                Ability(W(Min)),
                Ability(W(Min)),
                Ability(E(Max)),
                Attack,
                Ability(E(Max)),
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Ability(W(Max)),
                Ability(E(Max)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([Ability(E(Max)), Ability(R(Void)), Ability(Q(Void)), Attack])?
            .combo([
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Ability(W(Max)),
                Ability(R(Void)),
            ])?
            .combo([
                Ability(W(Max)),
                Ability(E(Max)),
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Ability(E(Max)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([
                Ability(E(Max)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Max)),
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .end()
    }
}
