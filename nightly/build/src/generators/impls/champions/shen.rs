use super::*;

impl Generator for Shen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Bonus Magic Damage", Min),
                ("Increased Bonus Damage", _1Min),
                ("Total Increased Damage", _1Max),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(Key::E, [("Physical Damage", Void)])
        .combos([
            &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack, Attack],
            &[
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(Q(Max)), Attack, Attack, Attack],
            &[Ability(Q(Max)), Attack, Ability(E(Void))],
            &[Ability(Q(Max)), Attack],
        ])?
        .end()
    }
}
