use super::*;

impl Generator for TahmKench {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, _1) /* Innate */, (2, _2) /* Innate [1] */])
            .modify(P(_2), |dmg| f![(dmg / 100 * AbilityPower)])?
            .merge_sum([P(_1), P(_2)], P(Void))?
            .ability(Key::Q, [(1, Min) /* Magic Damage */]);

        let passive = self[Q(Min)].damage.clone();

        self.clone_with(Q(Min), Q(Max), |dmg| f![(dmg) + passive])?
            .comment(Q(Max), "Damage Including Passive")?
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability_nth(1, Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
