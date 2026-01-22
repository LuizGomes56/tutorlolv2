use super::*;

impl Generator<ItemData> for StatBonus {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
