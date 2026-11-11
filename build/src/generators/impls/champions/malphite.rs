use super::*;

impl Generator for Malphite {
    fn generate(&mut self) -> MayFail {
        // Thunderclap passive bonus armor, as a ratio of armor
        let bonus_armor = self.ranks(Key::W, "Bonus Armor")?;

        self.constant(
            "THUNDERCLAP_BONUS_ARMOR",
            bonus_armor.iter().map(|v| v / 100.0).collect::<Vec<_>>(),
        )?
        .ability(Key::Q, [("Magic Damage", Void)])
        .ability(
            Key::W,
            [("Additional Physical Damage", _1), ("Physical Damage", _2)],
        )
        .ability(Key::E, [("Magic Damage", Void)])
        .ability(Key::R, [("Magic Damage", Void)])
        .damage_types([W(_1), W(_2)], Physical)?
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
