use super::*;

impl Generator for Udyr {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Min))?
            .take("Bonus Physical Damage On-Hit", Q(Void))?
            .take("Total Physical Damage", Q(Max))?
            .take("Magic Damage per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .damage_type(Q(Void), Physical)
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .end()
    }
}
