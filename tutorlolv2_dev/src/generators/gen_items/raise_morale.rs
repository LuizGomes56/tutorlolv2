use super::*;

impl Generator<ItemData> for RaiseMorale {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
