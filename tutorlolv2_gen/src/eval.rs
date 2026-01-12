use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Creates the `EvalIdent` and `Ctx` structs, associating
/// the appropriate names and numeric types that it will hold. This struct
/// is essential to the application since it is used to evaluate all the
/// generated closures contained in cache static variables
macro_rules! create_eval_struct {
    ($($value:ident),*$(,)?) => {
        pastey::paste! {
            /// Defines a standard type that implements trait [`core::fmt::Display`]
            /// and is used to create constant closures in the static variables of
            /// this module. For example:
            /// [`EvalIdent::QLevel`] is converted to: `ctx.q_level`
            #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
            #[repr(u8)]
            pub enum EvalIdent {
                $([<$value:camel>],)*
            }

            $(
                #[allow(non_upper_case_globals)]
                pub const [<$value:camel>]: EvalIdent = EvalIdent::[<$value:camel>];
            )*

            #[derive(Clone, Copy, Debug, Decode, Default, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
            #[repr(C)]
            pub struct Ctx {
                $(pub $value: f32,)*
            }

            impl ::core::fmt::Display for EvalIdent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        $(
                            Self::[<$value:camel>] => write!(f, concat!("ctx.", stringify!($value))),
                        )*
                    }
                }
            }
        }
    };
}

create_eval_struct!(
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
    ad,
    q_level,
    w_level,
    e_level,
    r_level,
);
