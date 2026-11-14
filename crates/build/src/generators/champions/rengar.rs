use super::*;

impl Generator for Rengar {
    fn generate(&mut self) -> MayFail {
        self.take("Additional Physical Damage", Q(Void))?
            .map_change(Q(Void), "Base damage")?
            .map_change(Q(Void), "AD ratio")?
            .map_change(Q(Void), "Unempowered AD ratio")?
            .take("Magic Damage", W(Void))?
            .map_change(W(Void), "Base damage")?
            .map_change(W(Void), "AP ratio")?
            .take("Physical Damage", E(Void))?
            .map_change(E(Void), "Bonus AD ratio")?
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
