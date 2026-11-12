use super::*;

impl GeneratorV2 for Taliyah {
    fn generate(&mut self) -> MayFail {
        self.take("Empowered Damage", Q(_1))?
            .take("Reduced Damage", Q(_2))?
            .take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage", E(Void))?
            .take("Detonation Magic Damage", E(_1))?
            .take("Total Maximum Detonation Damage", E(_2))?
            .end()
    }
}
