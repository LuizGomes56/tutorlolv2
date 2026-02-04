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
            #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Encode, Decode, Serialize, Deserialize)]
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
    ability_power,
    adaptative_damage,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    attack_speed,
    base_ad,
    base_armor,
    base_health,
    base_magic_resist,
    base_mana,
    bonus_ad,
    bonus_armor,
    bonus_health,
    bonus_magic_resist,
    bonus_mana,
    bonus_move_speed,
    crit_chance,
    crit_damage,
    current_health,
    current_mana,
    enemy_armor,
    enemy_bonus_armor,
    enemy_bonus_health,
    enemy_bonus_magic_resist,
    enemy_health,
    enemy_magic_resist,
    enemy_max_health,
    enemy_missing_health,
    level,
    q_level,
    w_level,
    e_level,
    r_level,
    magic_multiplier,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    max_health,
    max_mana,
    missing_health,
    physical_multiplier,
    randuin_effect,
    rocksolid_effect,
    stacks,
    steelcaps_effect,
);
