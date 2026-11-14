use super::*;

impl Generator for Zaahen {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage per Hit", Q(Void))?
            .take("Bonus Physical Damage", Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Subsequent Physical Damage", W(Void))?
            .take("Initial Physical Damage", W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Bonus Magic Damage", E(Void))?
            .take("Physical Damage", E(Min))?
            .take("Increased Physical Damage", E(Max))?
            .take("Physical Damage", R(Void))?
            .damage_type(E(Void), Magic)
            .damage_type(E(Min), Physical)
            .damage_type(E(Max), Physical)
            .end()
    }
}
