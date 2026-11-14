use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        // Mega Gnar stat increases are given from level 1 to level 18. Stats grow by
        // `per_level * 17` up to level 18, so their growth is the range over 17
        let mega_gnar = self.exception("Mega Gnar")?.to_string();

        for (name, pattern) in [
            (
                "HEALTH",
                regex!(r"([\d.]+) - ([\d.]+) \(based on level\) base health\b"),
            ),
            (
                "ATTACK_DAMAGE",
                regex!(r"([\d.]+) - ([\d.]+) \(based on level\) base attack damage\b"),
            ),
            (
                "ARMOR",
                regex!(r"([\d.]+) - ([\d.]+) \(based on level\) base armor\b"),
            ),
            (
                "MAGIC_RESIST",
                regex!(r"([\d.]+) - ([\d.]+) \(based on level\) base magic resistance\b"),
            ),
        ] {
            let [min, max] = mega_gnar.capture(pattern)?;

            self.constant(&format!("MEGA_{name}_BASE"), min)?
                .constant(&format!("MEGA_{name}_PER_LEVEL"), (max - min) / 17.0)?;
        }

        let [min, max] = mega_gnar.capture(regex!(
            r"loses ([\d.]+)% - ([\d.]+)% \(based on level\) bonus attack speed"
        ))?;

        self.constant("MEGA_ATTACK_SPEED_LOSS_PER_LEVEL", (max - min) / 17.0)?
            .take_in("Boomerang Throw", "Physical Damage", Q(Max))?
            .take_in("Boomerang Throw", "Reduced Damage", Q(Min))?
            .take_in("Boulder Toss", "Physical Damage", Q(Mega))?
            .take_in("Hyper", "Bonus Magic Damage", W(Void))?
            .take_in("Wallop", "Physical Damage", W(Mega))?
            .take_in("Hop", "Physical Damage", E(Void))?
            .take_in("Crunch", "Physical Damage", E(Mega))?
            .take("Increased Damage", R(Max))?
            .take("Physical Damage", R(Min))?
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
