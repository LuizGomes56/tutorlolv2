use super::*;

impl GeneratorV2 for Yunara {
    fn generate(&mut self) -> MayFail {
        self.take("Passive Bonus Magic Damage", Q(Void))?
            .take("Initial Magic Damage", W(Void))?
            .take("Linger Magic Damage per Tick", W(_1))?
            .take("Total Expanded Damage", W(_2))?
            .take(
                "[[File: Yunara Arc of Ruin.png|20px|border|link=]] Arc of Ruin Base Damage",
                R(Void),
            )?
            .damage_type(Q(Void), Magic)
            .damage_type(R(Void), Magic)
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
