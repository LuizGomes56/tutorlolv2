use super::*;

impl GeneratorV2 for Twitch {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Deadly Venom", 0, P(Min))?
            .derive(P(Max), P(Min), |dmg| f![(dmg) * 6])?
            .derive(P(_1Min), P(Max), |dmg| f![(dmg) * 6])?
            .derive(P(_1Max), P(_1Min), |dmg| f![(dmg) * 6])?
            .take("Base Physical Damage", E(Void))?
            .take("Maximum Mixed Damage", E(Max))?
            .take("Minimum Mixed Damage", E(Min))?
            .take("Physical Damage Per Stack", E(_1))?
            .combos([
                &[Attack, Attack, Ability(E(Void)), Attack, Attack],
                &[Attack, Attack, Attack, Ability(E(Void)), Attack, Attack],
                &[Attack, Attack, Attack, Ability(E(Void))],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Attack, Attack, Attack, Attack, Ability(E(Void))],
            ])?
            .end()
    }
}
