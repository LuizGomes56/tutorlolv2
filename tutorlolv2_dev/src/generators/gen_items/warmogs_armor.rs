use super::*;

impl Generator<ItemData> for WarmogsArmor {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
