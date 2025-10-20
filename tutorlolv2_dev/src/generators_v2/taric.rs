use super::*;

impl Generator for Taric {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![E, (0, 0, _1)];
    }
}
