use super::*;

impl Generator<Champion> for Singed {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1), (2, 1, _2)];
        ability![E, (0, 0, _1)];
    }
}
