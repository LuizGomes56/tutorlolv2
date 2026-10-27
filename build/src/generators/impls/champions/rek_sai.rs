use super::*;

impl Generator for RekSai {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                (0, Min), /* Bonus Physical Damage */
                (1, Max), /* Total Bonus Physical Damage */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .comment(Q(_1), "Prey Seeker while burrowed")?
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability_nth(
            1,
            Key::E,
            [
                (0, Void), /* Physical Damage */
                (1, _1),   /* True Damage */
            ],
        )
        .damage_type(E(_1), True)?
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .combos([
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
            ],
            &[Attack, Ability(Q(Max)), Attack, Attack, Ability(E(Void))],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
