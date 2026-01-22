use super::*;

impl Generator<ItemData>
    for SpellslingersShoes
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
