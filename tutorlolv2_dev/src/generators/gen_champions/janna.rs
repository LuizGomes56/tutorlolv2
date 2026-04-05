use super::*;

impl Generator<Champion> for Janna {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, Max), (0, 2, Min)]);
        self.ability(Key::W, [(0, 0, Void)]);

        self.attr(Area, [Q(Min), Q(Max)])?;
        self.combo([Ability(W(Void)), Attack, Ability(Q(Max)), Attack])?;

        self.end()
    }

    fn progress(&self) -> Progress {
        Stable
    }
}
