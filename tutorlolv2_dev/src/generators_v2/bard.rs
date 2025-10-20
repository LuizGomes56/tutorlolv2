use super::*;

impl Generator for Bard {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1)];
    }
}
