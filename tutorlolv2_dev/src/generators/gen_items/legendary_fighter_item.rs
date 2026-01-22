use super::*;

impl Generator<ItemData>
    for LegendaryFighterItem
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
