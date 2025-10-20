use super::*;

impl Generator for TwistedFate {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (1, 0, _1), (2, 0, _2), (5, 0, _3)];
        ability![E, (0, 1, _1)];
    }
}
