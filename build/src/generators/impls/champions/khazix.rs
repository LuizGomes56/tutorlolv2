use super::*;

impl Generator for Khazix {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability_nth(
                1,
                Key::Q,
                [
                    (0, Max), /* Isolated Target Physical Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .ability_nth(1, Key::W, [(1, Void) /* Physical Damage */])
            .ability_nth(1, Key::E, [(0, Void) /* Physical Damage */])
            .damage_types([W(Void), E(Void)], Physical)?
            .combo([
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(E(Void)),
            ])?
            .combo([Attack, Ability(W(Void)), Ability(E(Void)), Ability(Q(Max))])?
            .combo([Ability(E(Void)), Ability(Q(Max)), Ability(E(Void))])?
            .combo([
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
            ])?
            .combo([Ability(W(Void)), Ability(E(Void)), Ability(Q(Max)), Attack])?
            .combo([Ability(E(Void)), Ability(Q(Max)), Attack, Ability(W(Void))])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Max))])?
            .combo([Ability(W(Void)), Ability(Q(Max)), Attack])?
            .combo([Attack, Ability(W(Void)), Ability(Q(Max))])?
            .end()
    }
}
