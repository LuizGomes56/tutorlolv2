use super::*;

impl Generator for Samira {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Min))?
            // Increased by the target's missing health, up to twice the damage
            .take_nth("Innate", 2, P(Max))?
            .damage_type(P(Min), Magic)
            .damage_type(P(Max), Magic)
            // Each of the 6 bolts against an immobilized target
            .take_nth("Innate", 3, P(_1Min))?
            .derive(P(_1Max), P(_1Min), |bolt| f![6 * bolt])?
            .damage_type(P(_1Min), Physical)
            .damage_type(P(_1Max), Physical)
            .take("Physical Damage", Q(Void))?
            .take("Physical Damage per Hit", W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Physical Damage Per Shot", R(Min))?
            .map_change(R(Min), "Base damage per tick")?
            .take("Total Physical Damage", R(Max))?
            .comment(P(_1Min), "Passive against immobilized targets");

        let shot = self.damage(R(Min))?;

        // Up to 10 shots per enemy
        self.set(R(Max), f![10 * shot])
            .combos([
                &[
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max)), Attack],
            ])?
            .end()
    }
}
