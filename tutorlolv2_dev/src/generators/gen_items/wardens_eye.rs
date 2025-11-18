use super::*;

impl Generator<ItemData> for WardensEye {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
