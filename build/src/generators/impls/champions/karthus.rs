use super::*;

impl Generator for Karthus {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Isolated Enhanced Damage */
                (1, Min), /* Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Max), /* Damage Per Second */
                (1, Min), /* Magic Damage Per Tick */
            ],
        )
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .combos([
            &[
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
            ],
        ])?
        .end()
    }
}
