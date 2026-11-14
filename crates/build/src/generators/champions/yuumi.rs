use super::*;

impl Generator for Yuumi {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Minimum Bonus Magic Damage On-Hit", Q(_1Min))?
            .take("Increased Damage", Q(Max))?
            .map_change(Q(Max), "Empowered base damage")?
            .map_change(Q(Max), "Empowered AP ratio")?
            .take("Maximum Bonus Magic Damage On-Hit", Q(_1Max))?
            .take("Magic Damage per Hit", R(Void))?
            .take("Reduced Damage per Hit", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[Ability(R(Void)), Ability(Q(Max))],
                &[Attack, Attack, Ability(Q(Max))],
            ])?
            .end()
    }
}
