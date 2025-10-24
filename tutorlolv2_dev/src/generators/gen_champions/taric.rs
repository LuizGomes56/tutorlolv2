use super::*;

impl Generator<Champion> for Taric {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![E, (0, 0, _1)];
    }
}
