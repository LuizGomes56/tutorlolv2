use super::*;

impl Generator<ItemData> for RecurveBow {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
