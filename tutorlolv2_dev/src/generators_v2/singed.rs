use super::*;

impl Generator for Singed {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1), (2, 1, _2), (2, 2, _3)];
        ability![E, (0, 0, _1)];
    }
}
