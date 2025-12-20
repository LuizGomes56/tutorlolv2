#![no_std]

//! ### `tutorlolv2`
//!
//! This is a library that provides functions that evaluate the damage of every
//! ability of every champion, every item, and every rune in League of Legends. It
//! is focused on performance and it is fully statically typed, with wide support
//! to constant evaluation. In other words, there's no regex and any kind of string
//! parsing and expression-evaluation, meaning that everything can be evaluated at
//! compile time, with all of Rust's limitations related to it.
//!
//! - This crate depends on `alloc`, `const_sized_bit_set`, and `phf`
//! - **This crate does not depend on the standard library**, and has `no_std` on by default
//!
//! There are two main functions that are exported: [`realtime`] and [`calculator`].
//!
//! - [`realtime`] Receives deserialized data that comes directly from port 2999 of
//! the user's local machine livegame and returns the damage of every found item, rune,
//! and ability of the current player, against every enemy in the other team, and other
//! useful information about the game. You can enable feature `livegame` and use
//! `serde_json` to call that function directly.
//!
//!     ```rs
//!
//!     let port_2999_bytes = Client::get(
//!         "http://127.0.0.1:2999/liveclientdata/allgamedata"
//!     )
//!     .await?
//!     .bytes()
//!     .await?;
//!     // You can deserialize if feature "livegame" or "serde" is enabled
//!     let game = serde_json::from_slice(&port_2999_bytes)?;
//!     // If you don't activate those features, you can deserialize manually
//!     // or obtain such data from another source
//!     let data = realtime(&game).ok_or("Failed to run `tutorlolv2::realtime`")?;
//!
//!     // If you have feature "serde" activated, you can save it to a JSON file
//!     let json = serde_json::to_string_pretty(&data)?;
//!     std::fs::write("realtime_data.json", json)?;
//!
//!     // With feature "bincode", you can encode it to binary data and export
//!     // to your WebAssembly application
//!     let bin = bincode::encode_to_vec(&data, CFG)?;
//!     std::fs::write("realtime_data.bin", bin)?;
//!
//!     let number_of_enemies = data.enemies.len();
//!     for enemy in data.enemies {
//!         let Damages {
//!             abilities,
//!             items,
//!             runes,
//!             attacks
//!         } = enemy.damages;
//!         println!("Damage of attacks: {attacks:?}");
//!         // All instances below have the same type of `Box<[i32]>`, so you can
//!         // create a closure to simplify the code
//!         for (i, damage) in abilities.into_iter().enumerate() {
//!             let ability_id = data.abilities_meta[i].kind;
//!             println!("Damage for ability {ability_id:?} is {damage}");
//!         }
//!         for (i, damage) in items.into_iter().enumerate() {
//!             let item_id = data.items_meta[i].kind;
//!             println!("Damage for item {item_id:?} is {damage}");
//!         }
//!         for (i, damage) in runes.into_iter().enumerate() {
//!             let rune_id = data.runes_meta[i].kind;
//!             println!("Damage for rune {rune_id:?} is {damage}");
//!         }
//!         ...
//!     }
//!     ```
//!
//!     See the documentation for function [`realtime()`] for more details
//!
//! - [`calculator`] Receives data similar to function [`realtime`], but with the
//! library's own data types, and containing only the information needed to calculate
//! the current player and enemies state, and to achieve a more precise calculation,
//! accounting for item, rune, and champion exceptions. You have to provide the inputs
//! on your own since it is unrelated to Riot's API, and there's no public endpoint where
//! you can get data that fits this function's input.
//!
//! If you want to calculate the game information on your own, you can still use
//! this library to help you on this task, and avoid having to manually implement
//! all the logic to update your application on every patch. Everything used in
//! this library is re-exported, which means you can personalize the usage of the
//! calculations into your own data types, and recreate the functions [`realtime()`]
//! and [`calculator()`] to fit your needs. See the documentation for module
//! [`helpers`] for more details
//!
//! ### features
//! - `livegame` Enables serde's `Deserialize` traits to all types in the module [`riot`],
//! which allows you to get the data directly from Riot's API, deserialize it and call
//! the function [`realtime()`] afterwards (see the provided examples)
//! - `serde` Includes feature `livegame` and adds the traits `Serialize` and `Deserialize`
//! to every eligible struct. Notethat several structs have lifetime annotations, and some
//! have static lifetimes. Those do not implement `Deserialize`, so if you want to transform
//! it into a JSON file, you will not be able to get it back unless you create your own
//! derived data type to make it happen
//! - `bincode` Adds the traits `Encode` and `Decode` to every eligible struct. Structs
//! with lifetime annotations try to implement `BorrowDecode`, if no static lifetimes are
//! involved
//! - `no_std` is always on
//!
//! ### Warnings
//! - Since this library is entirely static, it needs to be recompiled if anything
//! in League of Legends gets updated. If a champion receives a rework, a new ability, or
//! a damage adjustment, this library have to be updated to a new version. Check the github
//! repository if you want to download it into your machine to not depend on its updates from
//! `crates.io`
//! - Riot's API is not guaranteed to be stable, so if anything changes, this entire library
//! may break
//! - It is recommended to add a panic handler to every non-constant function called from
//! this library, since they assume that the input data is always valid, that the
//! generated function prototypes are correct, and that oversized inputs might trigger
//! an stack overflow error

pub use calculator::calculator;
pub use realtime::realtime;
pub use tutorlolv2_gen::{
    AdaptativeType, AttackType, Attrs, ChampionId, DamageType, EvalContext, GameMap, ItemId,
    Position, RuneId,
};

pub mod constants {
    //! This module exports static variables that represent the data generated
    //! by the build script. All of those variables are arrays that can be indexed
    //! using their corresponding `id` enum types, which are the following
    //! - [`ChampionId`] indexes [`CHAMPION_CACHE`]
    //! - [`ItemId`] indexes [`ITEM_CACHE`]
    //! - [`RuneId`] indexes [`RUNE_CACHE`]
    //!
    //! If you're still in doubt of how to use this module, see the example below
    //! ```rs
    //! let my_champion = ChampionId::Neeko;
    //! let my_item = ItemId::BladeOfTheRuinedKing;
    //! let my_rune = RuneId::Electrocute;
    //!
    //! let neeko = CHAMPION_CACHE[my_champion as usize];
    //! let bork = ITEM_CACHE[my_item as usize];
    //! let electrocute = RUNE_CACHE[my_rune as usize];
    //! ```
    //!
    //! If you only want to use one single value, you don't need to use any of those
    //! arrays. Instead you can directly access each static variable by using its
    //! corresponding module, such as [`champions`], [`items`], or [`runes`].
    //!
    //! The code shown before can be simplified using:
    //! ```rs
    //! let neeko = tutorlolv2::champions::NEEKO;
    //! let bork = tutorlolv2::items::BLADE_OF_THE_RUINED_KING;
    //! let electrocute = tutorlolv2::runes::ELECTROCUTE;
    //! ```
    //!
    //! If you want to verify what information those static variables hold, check
    //! fields for the following structs
    //! - [`tutorlolv2_gen::CachedChampion`]
    //! - [`tutorlolv2_gen::CachedItem`]
    //! - [`tutorlolv2_gen::CachedRune`]

    pub use tutorlolv2_gen::{CHAMPION_CACHE, ITEM_CACHE, RUNE_CACHE};
}

pub mod champions {
    //! Holds static variables about champions, and functions
    //! that evaluate the damages of their abilities

    pub use tutorlolv2_gen::champions::*;
}

pub mod items {
    //! Holds static variables and functions that evaluate damages
    //! for every item

    pub use tutorlolv2_gen::items::*;
}

pub mod runes {
    //! Holds static variables and functions that evaluate damages
    //! for every rune

    pub use tutorlolv2_gen::runes::*;
}

extern crate alloc;

pub mod calculator;
pub mod const_eval;
pub mod helpers;
pub mod realtime;
pub mod riot;
