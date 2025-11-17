macro_rules! create_eval_ident {
    (u8($($bytes:ident),* $(,)?), f32($($floats:ident),* $(,)?)) => {
        tutorlolv2_types::paste! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum EvalIdent {
                $(
                    [<$bytes:camel>],
                )*
                $(
                    [<$floats:camel>],
                )*
            }

            $(
                #[allow(non_upper_case_globals)]
                pub const [<$bytes:camel>]: EvalIdent = EvalIdent::[<$bytes:camel>];
            )*
            $(
                #[allow(non_upper_case_globals)]
                pub const [<$floats:camel>]: EvalIdent = EvalIdent::[<$floats:camel>];
            )*

            #[derive(Default, Debug, Copy, Clone)]
            pub struct EvalContext {
                $(
                    pub $bytes: u8,
                )*
                $(
                    pub $floats: f32,
                )*
            }

            impl ::core::fmt::Display for EvalIdent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        $(
                            Self::[<$bytes:camel>]  => write!(f, concat!("ctx.", stringify!($bytes))),
                        )*
                        $(
                            Self::[<$floats:camel>] => write!(f, concat!("ctx.", stringify!($floats))),
                        )*
                    }
                }
            }
        }
    };
}

create_eval_ident!(
    u8(q_level, w_level, e_level, r_level),
    f32(
        level,
        chogath_stacks,
        veigar_stacks,
        nasus_stacks,
        smolder_stacks,
        aurelion_sol_stacks,
        thresh_stacks,
        kindred_stacks,
        belveth_stacks,
        adaptative_damage,
        physical_multiplier,
        magic_multiplier,
        steelcaps_effect,
        randuin_effect,
        rocksolid_effect,
        enemy_bonus_health,
        enemy_armor,
        enemy_max_health,
        enemy_health,
        enemy_current_health,
        enemy_missing_health,
        enemy_magic_resist,
        base_health,
        base_ad,
        base_armor,
        base_magic_resist,
        base_mana,
        bonus_ad,
        bonus_armor,
        bonus_magic_resist,
        bonus_health,
        bonus_mana,
        bonus_move_speed,
        armor_penetration_flat,
        armor_penetration_percent,
        magic_penetration_flat,
        magic_penetration_percent,
        max_mana,
        current_mana,
        max_health,
        current_health,
        armor,
        magic_resist,
        crit_chance,
        crit_damage,
        attack_speed,
        missing_health,
        ap,
        ad
    )
);
