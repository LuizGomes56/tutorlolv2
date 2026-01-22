use super::*;

impl Generator<ItemData>
    for MoonflairSpellblade
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
