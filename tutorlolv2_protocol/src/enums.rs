//! Enums for values Riot sends as strings, and for the closed value sets
//! stored in the database (spec D17).
//!
//! Open sets Riot can extend are declared with [`open_enum!`]: unknown values
//! are kept in `Other(String)` instead of being lost, and every encoding
//! (serde, bincode, Postgres text) uses the string form, so adding a variant
//! later never changes stored bytes.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Declares an enum over a set of strings that Riot may extend.
///
/// ```
/// tutorlolv2_protocol::open_enum!(pub enum Side { Blue = "BLUE", Red = "RED" });
/// assert_eq!(Side::from("BLUE"), Side::Blue);
/// assert_eq!(Side::from("GREEN").as_str(), "GREEN");
/// ```
#[macro_export]
macro_rules! open_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($variant:ident = $wire:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        $vis enum $name {
            $($variant,)+
            /// A value this version doesn't know yet, kept verbatim.
            Other($crate::__private::String),
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $wire,)+
                    Self::Other(value) => value.as_str(),
                }
            }
        }

        impl ::core::convert::From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $($wire => Self::$variant,)+
                    other => Self::Other(other.into()),
                }
            }
        }

        impl ::core::convert::From<$crate::__private::String> for $name {
            fn from(value: $crate::__private::String) -> Self {
                match value.as_str() {
                    $($wire => Self::$variant,)+
                    _ => Self::Other(value),
                }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl ::core::cmp::Eq for $name {}

        impl ::core::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
            }
        }

        impl ::core::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl ::core::hash::Hash for $name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.as_str().hash(state)
            }
        }

        impl $crate::__private::serde::Serialize for $name {
            fn serialize<S: $crate::__private::serde::Serializer>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::__private::serde::Deserializer<'de>>(deserializer: D) -> ::core::result::Result<Self, D::Error> {
                <$crate::__private::String as $crate::__private::serde::Deserialize>::deserialize(deserializer).map(Self::from)
            }
        }

        impl $crate::__private::bincode::Encode for $name {
            fn encode<E: $crate::__private::bincode::enc::Encoder>(&self, encoder: &mut E) -> ::core::result::Result<(), $crate::__private::bincode::error::EncodeError> {
                $crate::__private::bincode::Encode::encode(self.as_str(), encoder)
            }
        }

        impl<Context> $crate::__private::bincode::Decode<Context> for $name {
            fn decode<D: $crate::__private::bincode::de::Decoder<Context = Context>>(decoder: &mut D) -> ::core::result::Result<Self, $crate::__private::bincode::error::DecodeError> {
                <$crate::__private::String as $crate::__private::bincode::Decode<Context>>::decode(decoder).map(Self::from)
            }
        }

        $crate::__private::bincode::impl_borrow_decode!($name);
    };
}

/// Stores an open enum as Postgres `text`. Only applied inside this crate, so
/// callers of [`open_enum!`] never see a `sqlx` feature check.
#[cfg(feature = "sqlx")]
macro_rules! impl_sqlx_text {
    ($($name:ident),+) => {$(
        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <&str as sqlx::Type<sqlx::Postgres>>::type_info()
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <&str as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }

        impl sqlx::Encode<'_, sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                <&str as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.as_str(), buf)
            }
        }

        impl sqlx::Decode<'_, sqlx::Postgres> for $name {
            fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                <&str as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self::from)
            }
        }
    )+};
}

open_enum!(
    /// LCU `region-locale.region`.
    pub enum Region {
        Na = "NA",
        Br = "BR",
        Euw = "EUW",
        Eune = "EUNE",
        La1 = "LA1",
        La2 = "LA2",
        Oce = "OCE",
        Tr = "TR",
        Ru = "RU",
        Kr = "KR",
        Jp = "JP",
    }
);

open_enum!(
    /// Riot API platform routing value,
    /// e.g. Spectator-V5 `platformId`.
    pub enum Platform {
        Na1 = "NA1",
        Br1 = "BR1",
        Euw1 = "EUW1",
        Eun1 = "EUN1",
        La1 = "LA1",
        La2 = "LA2",
        Oc1 = "OC1",
        Tr1 = "TR1",
        Ru = "RU",
        Kr = "KR",
        Jp1 = "JP1",
    }
);

open_enum!(
    /// `gameData.gameMode` of port 2999 and Riot APIs. `KIWI` is ARAM: Mayhem,
    /// `CHERRY` is Arena.
    pub enum GameMode {
        Classic = "CLASSIC",
        Aram = "ARAM",
        Kiwi = "KIWI",
        PracticeTool = "PRACTICETOOL",
        Cherry = "CHERRY",
        Urf = "URF",
    }
);

open_enum!(
    /// `allPlayers[*].team`.
    pub enum Team {
        Order = "ORDER",
        Chaos = "CHAOS",
    }
);

open_enum!(
    /// Spectator-V5 `gameType` (`MATCHED`) and LCU end-of-game `gameType` (`MATCHED_GAME`).
    pub enum GameType {
        Matched = "MATCHED",
        MatchedGame = "MATCHED_GAME",
        Custom = "CUSTOM",
        CustomGame = "CUSTOM_GAME",
    }
);

#[cfg(feature = "sqlx")]
impl_sqlx_text!(Region, Platform, GameMode, Team, GameType);

/// Riot API regional routing value (ACCOUNT-V1 host).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encode, Decode)]
pub enum Regional {
    Americas,
    Europe,
    Asia,
    Sea,
}

impl Regional {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Americas => "americas",
            Self::Europe => "europe",
            Self::Asia => "asia",
            Self::Sea => "sea",
        }
    }
}

impl Region {
    /// Platform serving this LCU region. Only `NA` ⇒ `NA1` was observed (V7);
    /// the rest is Riot's standard routing table.
    pub fn platform(&self) -> Option<Platform> {
        Some(match self {
            Self::Na => Platform::Na1,
            Self::Br => Platform::Br1,
            Self::Euw => Platform::Euw1,
            Self::Eune => Platform::Eun1,
            Self::La1 => Platform::La1,
            Self::La2 => Platform::La2,
            Self::Oce => Platform::Oc1,
            Self::Tr => Platform::Tr1,
            Self::Ru => Platform::Ru,
            Self::Kr => Platform::Kr,
            Self::Jp => Platform::Jp1,
            Self::Other(_) => return None,
        })
    }
}

impl Platform {
    pub fn regional(&self) -> Option<Regional> {
        Some(match self {
            Self::Na1 | Self::Br1 | Self::La1 | Self::La2 => Regional::Americas,
            Self::Euw1 | Self::Eun1 | Self::Tr1 | Self::Ru => Regional::Europe,
            Self::Kr | Self::Jp1 => Regional::Asia,
            Self::Oc1 => Regional::Sea,
            Self::Other(_) => return None,
        })
    }

    /// Host label of `https://{host}.api.riotgames.com`.
    pub fn host(&self) -> Option<&'static str> {
        Some(match self {
            Self::Na1 => "na1",
            Self::Br1 => "br1",
            Self::Euw1 => "euw1",
            Self::Eun1 => "eun1",
            Self::La1 => "la1",
            Self::La2 => "la2",
            Self::Oc1 => "oc1",
            Self::Tr1 => "tr1",
            Self::Ru => "ru",
            Self::Kr => "kr",
            Self::Jp1 => "jp1",
            Self::Other(_) => return None,
        })
    }
}

impl GameType {
    pub fn is_matched(&self) -> bool {
        matches!(self, Self::Matched | Self::MatchedGame)
    }
}

/// Why a recording ended. Postgres type `end_reason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(
    feature = "sqlx",
    derive(sqlx::Type),
    sqlx(type_name = "end_reason", rename_all = "snake_case")
)]
pub enum EndReason {
    GameEndEvent,
    LcuEndOfGame,
    LcuLeftGame,
    Superseded,
    Timeout,
}

/// Outcome of a game for the recording's player. Postgres type `game_result`.
/// Deserializes from Riot's `"Win"` / `"Lose"` (`GameEnd.Result`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(
    feature = "sqlx",
    derive(sqlx::Type),
    sqlx(type_name = "game_result", rename_all = "snake_case")
)]
pub enum GameResult {
    Win,
    Lose,
    Remake,
    Unknown,
}

/// Visibility of a recording as the API reports it (no code when `Private`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShareMode {
    Private,
    Live,
    Public,
}

/// Mode of an existing share code. Postgres type `share_mode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(
    feature = "sqlx",
    derive(sqlx::Type),
    sqlx(type_name = "share_mode", rename_all = "snake_case")
)]
pub enum CodeMode {
    Live,
    Public,
}

impl From<CodeMode> for ShareMode {
    fn from(mode: CodeMode) -> Self {
        match mode {
            CodeMode::Live => Self::Live,
            CodeMode::Public => Self::Public,
        }
    }
}
