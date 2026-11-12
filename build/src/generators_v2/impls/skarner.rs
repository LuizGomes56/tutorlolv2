use super::*;

impl GeneratorV2 for Skarner {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .set(P(Void), {
                f![(0.05 + 0.04 / 17 * (Level - 1)) * EnemyMaxHealth]
            })
            .take("Bonus Physical Damage per Hit", Q(Min))?
            .take("Total Bonus Physical Damage", Q(Max))?
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .damage_type(W(Void), Magic)
            .combos([
                &[Ability(R(Void)), Ability(W(Void))],
                &[Ability(E(Void)), Ability(R(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(W(Void)), Attack, Attack],
            ])?
            .end()
    }
}
