use super::*;

impl Generator<Champion> for Singed {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(2, 0, _1), (2, 1, _2)],
        );
        self.ability(E, [(0, 0, _1)]);
        self.end()
    }
}
