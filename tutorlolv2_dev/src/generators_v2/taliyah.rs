use super::*;

impl Generator for Taliyah {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2), (0, 2, _3), (3, 0, _4)];
        ability![E, (0, 0, _1), (1, 0, _2), (2, 0, _3)];
    }
}
