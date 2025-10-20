use super::*;

impl Generator for Ivern {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (2, 0, _1), (3, 0, _2)];
        ability![E, (1, 0, _1)];
    }
}
