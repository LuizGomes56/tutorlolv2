use super::*;

impl Generator for Azir {
    fn generate(&mut self) -> MayFail {
        // 0 at level 1, then + 8 per level starting at level 10
        let level_bonus = stringify!(match ctx.level {
            ..=9 => 0,
            10 => 8,
            11 => 16,
            12 => 24,
            13 => 32,
            14 => 40,
            15 => 48,
            16 => 56,
            17 => 64,
            18.. => 72,
        });

        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .modify(W(Void), |_| {
                f![(level_bonus)
                    + 50
                    + 15 * (WLevel - 1)
                    + (0.35 + 0.075 * (WLevel - 1)) * AbilityPower]
            })?
            .comment(W(Void), "Sand Soldier attack")?
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([Ability(E(Void)), Ability(Q(Void)), Ability(R(Void))])?
            .combo([Ability(W(Void)), Ability(E(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(W(Void)), Ability(E(Void)), Ability(Q(Void))])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Void)), Attack])?
            .combo([
                Ability(R(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
            ])?
            .end()
    }
}
