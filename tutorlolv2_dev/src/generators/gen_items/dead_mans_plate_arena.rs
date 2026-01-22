use super::*;

impl Generator<ItemData> for DeadMansPlateArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
