use super::*;

impl Generator for Kassadin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Increased Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1Min), /* Bonus Damage Per Stack */
                    (1, Min),   /* Magic Damage */
                    (2, _1Max), /* Maximum Bonus Damage */
                    (3, Max),   /* Maximum Magic Damage */
                ],
            )
            .combo([
                Ability(R(Max)),
                Ability(W(Void)),
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
            ])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Attack, Ability(W(Void))])?
            .combo([
                Ability(R(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(R(Min)),
                Attack,
            ])?
            .combo([Ability(E(Void)), Attack, Ability(W(Void)), Ability(Q(Void))])?
            .combo([
                Ability(R(Max)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Max)),
                Attack,
                Ability(W(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(R(Max)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(R(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
            ])?
            .combo([Ability(E(Void)), Ability(Q(Void))])?
            .combo([Ability(R(Max)), Ability(E(Void)), Ability(Q(Void))])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Ability(R(Max))])?
            .end()
    }
}
