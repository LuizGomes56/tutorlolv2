use super::*;

impl Generator<Champion> for Pyke {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1)];
        ability![E, (1, 0, _1)];
    }
}
