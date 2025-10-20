use super::*;

impl Generator for Aphelios {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![P, (2, 0, _1)];
    }
}
