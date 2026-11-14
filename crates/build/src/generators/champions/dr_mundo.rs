use super::*;

impl Generator for DrMundo {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Damage", Q(Min))?
            .take("Magic Damage", Q(Max))?
            .take("Magic Damage per Tick", W(_1))?
            .take("Magic Damage", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Minimum Bonus Physical Damage", E(Min))?
            .take("Maximum Bonus Physical Damage", E(Max))?
            .damage_type(E(Min), Physical)
            .damage_type(E(Max), Physical)
            .end()
    }
}
