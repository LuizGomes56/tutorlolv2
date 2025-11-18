use super::*;

impl Generator<ItemData> for DeadMansPlate {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
