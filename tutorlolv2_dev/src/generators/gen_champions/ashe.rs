use super::*;

impl Generator<Champion> for Ashe {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, Min), (0, 2, Max)])
            .ability(Key::W, [(0, 1, Void)])
            .ability(Key::R, [(0, 0, Void)])
            .attr(Area, [R(Void), W(Void)])?
            .combo([Attack, Ability(W(Void)), Ability(R(Void)), Attack])?
            .progress(Stable);

        self.end()
    }
}
