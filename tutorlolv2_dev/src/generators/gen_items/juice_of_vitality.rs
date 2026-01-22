use super::*;

impl Generator<ItemData> for JuiceOfVitality {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
