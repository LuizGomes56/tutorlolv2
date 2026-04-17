use super::*;

impl Generator<Champion> for Janna {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, Max), (0, 2, Min)])
            .ability(Key::W, [(0, 0, Void)])
            .attr(Area, [Q(Min), Q(Max)])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Max)), Attack])?
            .progress(Stable);

        self.end()
    }
}
