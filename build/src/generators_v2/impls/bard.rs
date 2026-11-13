use super::*;

impl GeneratorV2 for Bard {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate - Meeps", 0, P(Void))?
            .comment(
                P(Void),
                "Meeps bonus magic damage, Stacks = Chimes collected",
            )
            .take("Magic Damage", Q(Void))?
            .combos([
                &[Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
