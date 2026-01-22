use super::*;

impl Generator<ItemData> for BountyOfWorlds {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
