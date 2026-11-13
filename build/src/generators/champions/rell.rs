use super::*;

impl Generator for Rell {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Bonus Magic Damage", W(_1))?
            .take("Bonus Magic Damage", E(Void))?
            .take("Magic Damage Per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .end()
    }
}
