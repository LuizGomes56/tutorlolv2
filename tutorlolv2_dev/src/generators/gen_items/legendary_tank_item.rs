use super::*;

impl Generator<ItemData> for LegendaryTankItem {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
