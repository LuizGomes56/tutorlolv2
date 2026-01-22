use super::*;

impl Generator<ItemData> for LegendaryTankItem {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
