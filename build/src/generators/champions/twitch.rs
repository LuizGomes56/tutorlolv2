use super::*;

impl Generator for Twitch {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Deadly Venom", 0, P(Min))?
            .take_nth("Deadly Venom", 1, P(Max))?
            .take_nth("Deadly Venom", 2, P(_1Min))?
            .take_nth("Deadly Venom", 3, P(_1Max))?
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
