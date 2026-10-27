use super::*;

impl Generator for Sylas {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(3, Min) /* Unshackled */])
            .modify(P(Min), |_| f![0.4 * AttackDamage + 0.2 * AbilityPower])?
            .comment(P(Min), "Secondary Target Damage")?
            .clone_to(P(Min), P(Max), f![1.3 * AttackDamage + 0.3 * AbilityPower])?
            .comment(P(Max), "Primary Target Damage")?
            .ability(
                Key::Q,
                [
                    (0, Min), /* Magic Damage */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .combos([
                &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(E(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
