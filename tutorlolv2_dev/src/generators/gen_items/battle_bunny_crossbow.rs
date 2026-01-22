use super::*;

impl Generator<ItemData> for BattleBunnyCrossbow {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
