use super::*;

impl GeneratorV2 for Malphite {
    fn generate(&mut self) -> MayFail {
        // Thunderclap passive bonus armor, as a ratio of armor
        let bonus_armor = match self.value("Bonus Armor", Key::W)? {
            Formula::Product(factors) => factors.iter().find_map(Formula::default_values),
            other => other.default_values(),
        }
        .ok_or("[Malphite] Bonus Armor has no values")?;

        self.constant("THUNDERCLAP_BONUS_ARMOR", bonus_armor)?
            .take("Magic Damage", Q(Void))?
            .take("Additional Physical Damage", W(_1))?
            .take("Physical Damage", W(_2))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .damage_type(W(_1), Physical)
            .damage_type(W(_2), Physical)
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(_1)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(_1)),
                ],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(W(_1)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(_1)),
                    Attack,
                ],
            ])?
            .end()
    }
}
