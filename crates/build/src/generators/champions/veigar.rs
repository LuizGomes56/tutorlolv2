use super::*;

impl Generator for Veigar {
    fn generate(&mut self) -> MayFail {
        let [ap_per_stack] = self
            .description(Key::P, "Phenomenal Evil")?
            .capture(regex!(r"Veigar gains\s+([\d.]+) ability power"))?;

        self.constant("PHENOMENAL_EVIL_AP_PER_STACK", ap_per_stack)?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Minimum Magic Damage", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Max))],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[Ability(W(Void)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
