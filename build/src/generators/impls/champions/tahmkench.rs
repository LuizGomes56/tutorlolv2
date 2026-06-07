use super::*;

impl Generator for TahmKench {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .modify(P(_2), |dmg| dmg.div(100).times(AbilityPower).parenthesize())?
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability(Key::Q, [(1, Min) /* Magic Damage */]);

        let passive = self[Q(Min)].damage.clone();

        self.clone_with(Q(Min), Q(Max), |dmg| dmg.parenthesize().plus(&passive))?
            .comment(Q(Max), "Damage Including Passive")?
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability_nth(1, Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
