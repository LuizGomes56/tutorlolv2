use super::*;

impl Generator<ItemData> for WanderingStorms {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
