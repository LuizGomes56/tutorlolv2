use super::*;

impl Generator for Skarner {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Bonus Physical Damage per Hit", Q(Min))?
            .map_change(Q(Min), "''Shattered Earth'' base damage")?
            .take("Total Bonus Physical Damage", Q(Max))?
            .take("Physical Damage", Q(Void))?
            .map_change(Q(Void), "''Upheaval'' base damage")?
            .map_change(Q(Void), "Final attack / Upheaval target health ratio")?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .damage_type(W(Void), Magic);

        // 3 empowered attacks
        let (mode, per_hit) = self.map_value(Key::Q, "''Shattered Earth'' base damage")?;
        self.map_base(Q(Max), mode, per_hit.times(3.0))?
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
