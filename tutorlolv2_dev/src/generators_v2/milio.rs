use super::*;

impl Generator for Milio {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (3, 0, _1)];
    }
}
