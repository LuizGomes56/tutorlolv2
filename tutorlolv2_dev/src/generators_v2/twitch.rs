use super::*;

impl Generator for Twitch {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![E, (1, 0, _1), (2, 0, _2), (2, 1, _3), (2, 2, _4)];
        ability![R, (0, 0, _1)];
    }
}
