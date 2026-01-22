use super::*;

impl Generator<ItemData>
    for SuperMechPowerField
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
