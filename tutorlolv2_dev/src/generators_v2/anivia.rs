use super::*;

impl Generator for Anivia {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (2, 0, _2), (2, 2, _3)];
        ability![E, (0, 0, _1), (0, 1, _2)];
        ability![R, (0, 0, _1), (3, 0, _2)];
    }
}
