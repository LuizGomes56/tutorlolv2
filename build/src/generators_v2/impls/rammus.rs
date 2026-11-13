use super::*;

impl GeneratorV2 for Rammus {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Active", W(Void))?
            .take("Magic Damage", R(Void))?
            .end()
    }
}
