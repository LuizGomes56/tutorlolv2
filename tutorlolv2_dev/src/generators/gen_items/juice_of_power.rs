use super::*;

impl Generator<ItemData> for JuiceOfPower {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
