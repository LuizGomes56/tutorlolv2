use super::*;

impl Generator for Varus {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Physical Damage", Q(Min))?
            .take("Minimum Reduced Damage", Q(_1Min))?
            .take("Maximum Physical Damage", Q(Max))?
            .take("Maximum Reduced Damage", Q(_1Max))?
            .take("Active Minimum Magic Damage", W(Min))?
            .take("Bonus Magic Damage", W(_1Min))?
            .take("Active Maximum Magic Damage", W(Max))?
            .take("[[File:Varus Piercing Arrow.png|20px|border|link=]] Maximum Bonus Magic Damage at Max Stacks", W(_1Max))?
            .take("Physical Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
            &[
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(W(Max)),
                Ability(Q(Max)),
            ],
            &[
                Ability(R(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[Ability(E(Void)), Ability(R(Void)), Ability(Q(Max))],
            &[Attack, Attack, Attack, Ability(E(Void)), Attack],
            &[Attack, Ability(E(Void)), Attack, Attack, Ability(Q(Max))],
        ])?
            .end()
    }
}
