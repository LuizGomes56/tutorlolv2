use super::*;

impl GeneratorV2 for Aphelios {
    fn generate(&mut self) -> MayFail {
        self.take_in("Calibrum", "description2", P(_1))?
            .damage_type(P(_1), Physical)
            .comment(P(_1), "Calibrum mark bonus damage per mark consumed")
            .take_nth("description3", 0, P(_2))?
            .damage_type(P(_2), Physical)
            .comment(
                P(_2),
                "Crescendum bonus damage, Stacks = Chakrams (up to 20)",
            )
            // Of the Sentry, the 5th weapon's page
            .take_nth("Active", 4, Q(Void))?
            .damage_type(Q(Void), Physical)
            .comment(Q(Void), "Pet: Crescendum Sentry basic attack")
            .combos([
                &[Ability(Q(Void)), Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Ability(Q(Void)), Attack, Attack],
                &[Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack, Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
