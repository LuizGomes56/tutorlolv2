use super::*;

impl Generator<ItemData> for NegatronCloakArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
