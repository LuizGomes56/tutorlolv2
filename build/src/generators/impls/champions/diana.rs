use super::*;

impl Generator for Diana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate - Moonsilver Blade */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage per Orb */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1),  /* Bonus Damage Per Champion */
                    (1, Min), /* Magic Damage */
                    (3, Max), /* Total Damage Vs. 5 Champions */
                ],
            )
            .combo([
                Ability(R(Max)),
                Ability(Q(_1)),
                Ability(W(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(W(Max)),
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(Q(_1)),
                Attack,
            ])?
            .combo([
                Ability(Q(_1)),
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Ability(R(Max)),
                Attack,
                Ability(E(Void)),
                Ability(W(Max)),
                Attack,
            ])?
            .combo([
                Ability(W(Max)),
                Attack,
                Ability(R(Max)),
                Attack,
                Ability(E(Void)),
                Ability(Q(_1)),
                Attack,
            ])?
            .combo([
                Ability(Q(_1)),
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(W(Max)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(W(Max)), Ability(E(Void)), Ability(Q(_1))])?
            .combo([
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(W(Max)),
                Ability(Q(_1)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(_1)),
                Ability(E(Void)),
                Ability(W(Max)),
                Attack,
                Ability(R(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(W(Max)),
                Attack,
                Attack,
                Ability(Q(_1)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Attack,
                Attack,
                Ability(Q(_1)),
                Ability(E(Void)),
                Attack,
                Ability(W(Max)),
                Ability(E(Void)),
            ])?
            .combo([Attack, Ability(W(Max)), Ability(Q(_1)), Attack, Attack])?
            .end()
    }
}
