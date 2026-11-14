use super::*;

impl Generator for Garen {
    fn generate(&mut self) -> MayFail {
        // Spins without bonus attack speed
        let [spins] = self
            .description(Key::E, "Active")?
            .capture(regex!(r"spins his sword around himself (\d+) \("))?;

        self.take("Bonus Physical Damage", Q(Void))?
            .take("Physical Damage Per Spin", E(Min))?
            .take("Critical Strike Damage Per Spin", E(_1))?
            .derive(E(Max), E(Min), |e| f![spins * (e)])?
            .comment(E(Max), &format!("Total damage of {spins} spins"))
            .take("True Damage", R(Void))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Ability(R(Void))],
                &[Attack, Ability(Q(Void)), Ability(E(Max)), Attack],
                &[Ability(E(Max)), Attack, Ability(Q(Void)), Ability(R(Void))],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
