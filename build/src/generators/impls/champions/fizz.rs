use super::*;

impl Generator for Fizz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .damage_type(Q(Void), Magic)?
            .ability(
                Key::W,
                [
                    (0, Void), /* Active Magic Damage */
                    (1, _1),   /* Active On-Hit Magic Damage */
                    (3, _2),   /* Total Passive Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (2, Min), /* Guppy Damage */
                    (0, _1),  /* Chomper Damage */
                    (1, Max), /* Gigalodon Damage */
                ],
            )
            .combo([
                Ability(Q(Void)),
                Ability(R(Max)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
            ])?
            .combo([Ability(E(Void)), Attack, Ability(W(Void)), Ability(Q(Void))])?
            .combo([Ability(Q(Void)), Ability(R(Max))])?
            .combo([Ability(E(Void)), Ability(W(Void)), Ability(Q(Void))])?
            .combo([
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(R(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(R(Max)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(R(Max)),
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
            ])?
            .end()
    }
}
