use super::*;

impl Generator<ItemData> for GuardiansAmulet {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
