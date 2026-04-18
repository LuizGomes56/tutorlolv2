use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 1, Max), (0, 2, Min)])
            .ability(Key::W, [(0, 0, Void)])
            .attr(Area, [Q(Min), Q(Max)])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Max)), Attack])?
            .progress(Stable)
            .end()
    }
}
