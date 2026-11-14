use super::*;

impl Generator for MonkeyKing {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Clone Outgoing Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Maximum Total Physical Damage", R(_1Max))?
            .take("Physical Damage Per Tick", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .map_change(R(Min), "Base damage per tick")?;

        // Cyclone ticks every 0.25 seconds
        let (mode, per_second) = self.map_value(Key::R, "AD ratio per second")?;
        let tick = self
            .map_ratio(R(Min), mode, &[AttackDamage], per_second.times(0.25))?
            .damage(R(Min))?;

        // 8 ticks over its 2 seconds, and 16 when it is cast twice
        self.set(R(Max), f![8 * tick])
            .set(R(_1Max), f![16 * tick])
            .damage_type(W(Void), Physical)
            .damage_type(R(Min), Physical)
            .damage_type(R(Max), Physical)
            .damage_type(R(_1Max), Physical)
            .edit(W(Void), |ratio| f![ratio * AttackDamage])
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[Ability(Q(Void)), Ability(W(Void))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
