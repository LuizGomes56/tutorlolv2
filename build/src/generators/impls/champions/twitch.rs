use super::*;

impl Generator for Twitch {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Deadly Venom", Min)])
            .clone_with(P(Min), P(Max), |dmg| f![(dmg) * 6])?
            .clone_with(P(Max), P(_1Min), |dmg| f![(dmg) * 6])?
            .clone_with(P(_1Min), P(_1Max), |dmg| f![(dmg) * 6])?
            .ability(
                Key::E,
                [
                    ("Base Physical Damage", Void),
                    ("Maximum Mixed Damage", Max),
                    ("Minimum Mixed Damage", Min),
                    ("Physical Damage Per Stack", _1),
                ],
            )
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
