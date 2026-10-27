use super::*;

impl Generator for Xayah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min),   /* Physical Damage Per Hit */
                (1, _1Min), /* Reduced Damage per Hit */
                (2, Max),   /* Total Physical Damage */
                (3, _1Max), /* Total Reduced Damage */
            ],
        )
        .ability(Key::E, [(1, Void) /* Physical Damage Per Feather */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .combos([
            &[Ability(R(Void)), Ability(E(Void)), Attack, Attack],
            &[Ability(Q(Max)), Attack, Ability(E(Void))],
            &[
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[Attack, Ability(Q(Max)), Attack, Ability(E(Void))],
            &[Ability(R(Void)), Ability(E(Void))],
        ])?
        .end()
    }
}
