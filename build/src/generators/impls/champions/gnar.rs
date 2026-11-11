use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        // Mega Gnar stat increases are given from level 1 to level 18. Stats grow by
        // `per_level * 17` up to level 18, so their growth is the range over 17
        let mega_gnar = self.exception("Mega Gnar")?.to_string();

        for (name, stat) in [
            ("HEALTH", "base health"),
            ("ATTACK_DAMAGE", "base attack damage"),
            ("ARMOR", "base armor"),
            ("MAGIC_RESIST", "base magic resistance"),
        ] {
            let [min, max] =
                mega_gnar.capture(&format!(r"([\d.]+) - ([\d.]+) \(based on level\) {stat}\b"))?;

            self.constant(&format!("MEGA_{name}_BASE"), min)?
                .constant(&format!("MEGA_{name}_PER_LEVEL"), (max - min) / 17.0)?;
        }

        let [min, max] = mega_gnar
            .capture(r"loses ([\d.]+)% - ([\d.]+)% \(based on level\) bonus attack speed")?;

        self.constant("MEGA_ATTACK_SPEED_LOSS_PER_LEVEL", (max - min) / 17.0)?
            .ability_nth(
                1,
                Key::Q,
                [("Physical Damage", Max), ("Reduced Damage", Min)],
            )
            .ability(Key::Q, [("Physical Damage", Mega)])
            .ability_nth(1, Key::W, [("Bonus Magic Damage", Void)])
            .ability(Key::W, [("Physical Damage", Mega)])
            .ability_nth(1, Key::E, [("Physical Damage", Void)])
            .ability(Key::E, [("Physical Damage", Mega)])
            .ability(
                Key::R,
                [("Increased Damage", Max), ("Physical Damage", Min)],
            )
            .combos([
                &[Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
