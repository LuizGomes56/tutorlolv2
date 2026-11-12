use super::*;

impl GeneratorV2 for Karma {
    fn generate(&mut self) -> MayFail {
        self.take_in("Inner Flame", "Magic Damage", Q(Void))?
            .take("Total Magic Damage", Q(_1))?
            .comment(Q(_1), "Mantra empowered impact")
            .take("Total Damage", Q(_2))?
            .comment(Q(_2), "Mantra empowered impact and Soulflare")
            .take("Magic Damage", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .combos([
                &[Ability(Q(Void)), Ability(W(Max)), Attack, Attack, Attack],
                &[
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(Max)), Attack, Ability(Q(Void)), Attack],
                &[Ability(W(Max)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(W(Max)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
