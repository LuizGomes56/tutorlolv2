use super::*;

impl Generator for Zilean {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
    }
}
