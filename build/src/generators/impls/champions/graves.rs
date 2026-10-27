use super::*;

impl Generator for Graves {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Physical Damage */
                (1, Max), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, Void), /* Physical Damage */
                (1, _1),   /* Reduced Damage */
            ],
        )
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(Q(Max)), Attack, Ability(R(Void))],
            &[Ability(W(Void)), Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack],
            &[Attack, Attack, Ability(Q(Max)), Attack],
            &[Ability(W(Void)), Attack, Attack, Ability(Q(Max)), Attack],
        ])?
        .end()
    }
}
