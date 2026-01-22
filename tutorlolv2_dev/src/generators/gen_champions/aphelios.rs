use super::*;

impl Generator<Champion> for Aphelios {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(P, [(2, 0, _1)]);
        self.end()
    }
}
