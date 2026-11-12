use super::*;

impl GeneratorV2 for Rammus {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Bonus Armor", W(Void))?
            .set(W(Void), f![15 + 0.1 * Armor + 0.1 * MagicResist])
            .take("Magic Damage", R(Void))?
            .end()
    }
}
