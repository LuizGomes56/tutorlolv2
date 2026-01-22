use super::*;

impl Generator<ItemData> for ClothArmor {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
