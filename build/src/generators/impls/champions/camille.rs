use super::*;

impl Generator for Camille {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Bonus Physical Damage */
                (2, Max), /* Increased Mixed Damage */
            ],
        )
        .damage_type(Q(Min), Physical)?
        .damage_type(Q(Max), Mixed)?
        .ability(Key::W, [(3, Min) /* Physical Damage */])
        .clone_with(W(Min), W(Max), |w| {
            f![(w) + (0.07 + 0.005 * (WLevel - 1) + 0.00025 * BonusAd) * EnemyMaxHealth]
        })?
        .comment(W(Max), "Outer cone damage")?
        .ability(Key::E, [(1, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Bonus Magic Damage */])
        .combos([
            &[
                Ability(E(Void)),
                Ability(W(Max)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(W(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(W(Max)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(W(Max)),
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Ability(W(Max)),
            ],
            &[Ability(E(Void)), Ability(E(Void))],
        ])?
        .end()
    }
}
