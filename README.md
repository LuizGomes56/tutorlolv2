## TutorLoLv2 - League of Legends library

**`tutorlolv2`** is a highly optimized library for the game League of Legends that calculates the damages of all abilities, items, runes, passives for every champion, updating automatically every new game patch. Also, it has great support to compile-time evaluation (const fn) and does not depend on the Rust's standard library `#![no_std]`

### Function `realtime` 

Takes as function argument a parsed struct that can be created from the JSON object provided by Riot's API on port 2999 while some player is playing a League of Legends game, for any game mode.

- The official endpoint is located at the following url, only available if the current machine has an ongoing match being played (replays do not work)
    - https://127.0.0.1:2999/liveclientdata/allgamedata
    - To see an example of how this data look like, check the following url:
    https://static.developer.riotgames.com/docs/lol/liveclientdata_sample.json

You can recover bytes from that URL directly and use `serde_json` to deserialize it and call the exported function `realtime` (see example in the `#[examples]` section). This is the cheapest way to use this library and get all the necessary information
    
Each call of this function takes about **80μs** and returns an average payload size of **70kB**, which can be reduced to an average of **25kB** when encoded with bincode. It can also be transformed to JSON, but the result payload will be much larger and slower to send to the frontend application
    
### Example of usage
```rs
const URL: &str = "https://127.0.0.1:2999/liveclientdata/allgamedata";

// Call Riot's API if there's any active game
let request = reqwest::Request::get(URL).await?;

// Get bytes from that port, if any
let port_2999_bytes = request.bytes().await?;

// Deserialize that data using serde_json
let deserialized = serde_json::from_slice(&port_2999_bytes)?;

// Call the realtime function
let data = tutorlolv2::realtime(&deserialized)
    .ok_or("Error on function realtime");

// You can turn it to a JSON string
let json = serde_json::to_string(&data)?;

// Or encode with bincode
let cfg = bincode::config::standard();
let bytes = bincode::encode_to_vec(data, cfg)?;
```

The variable `data` holds all the information of damages against every player in the enemy team, their names, and some metadata about the final calculation. The returned struct returns references to the deserialized JSON object to avoid cloning data

## Project structure

This project is split in several different modules or crates, where each do part of the work. Most of these modules help generate the crate `tutorlolv2_gen`, which contains closures and enum definitions that are used in the core library and frontend application

### `tutorlolv2_avif`

This module is used to convert all images downloaded from Riot's API to the `.avif` encoding, which help reduce the total size of all images by 66%. 

By now, all images are embedded in the final compiled binary, which means there's no need to save them to a folder and move them to the AWS server, but it causes the final binary to be over 200MB in size, and requires recompilation to add new images

### `tutorlolv2_dev`

Has several "generator files", which read cache files the following directories 
- `cache/riot/(champions | items)`
- `cache/meraki/(champions | items)` 

With the data in those folders, the useful information is gathered and new JSON files are generated on `internal/(champions | items)`, which will be read by `tutorlolv2_build` module and generate closures, structs, enums, static and constant variables. This method makes the evaluation speed of all functions much faster and type-safe than if we used Regex or other string-parsing method. However, the cost of this approach is that it needs the program to be recompiled every patch

### Example of generator file

As said before, they read data in the cache folder and generate a new JSON file with useful information about the current champion/item/rune as well as its damage formula

```rs
use super::*;

// Mark that this file does not need to be regenerated
// every new patch because this code does not tend to
// change very often
// #![stable]

// Every generator implements the trait `Generator`
impl Generator<Champion> for Neeko {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        // On cache file at ability "Q", we have at
        // "effect" array positions (0, 2, 2), at
        // levelings (0, 0, 1) the formula for the
        // damage of some abilities, named (_1, Min, Max)
        self.ability(Q, [(0, 0, _1), (2, 0, Min), (2, 1, Max)]);
        self.ability(W, [(1, 0, Void)]);
        self.ability(E, [(0, 0, Void)]);
        self.ability(R, [(2, 0, Void)]);
        // Q::_1, Q::Min, Q::Max, E::Void deal area damage
        self.attr(Area, dynarr!(Q::_1, Q::Min, Q::Max, E::Void, R::Void))?;
        // If it gets in here, the generator have run with
        // no errors found
        self.end()
    }
}
```

After this function is called, it generates a JSON file at location `internal/champions/Neeko.json` that look like the following

```jsonc
{
  "name": "Neeko",
  "adaptative_type": "Magic",
  "attack_type": "Ranged",
  "positions": ["Middle", "Support"],
  "stats": {
    "health": {
      "flat": 610.0,
      "perLevel": 104.0
    },
    // ...
  },
  "abilities": [
    [
        // Deserializes to Q::_1
        { "type": "Q", "name": "_1" },
        // Data about ability Q::1
        {
            "name": "Blooming Burst",
            "damage_type": "Magic",
            "attributes": "Undefined",
            // Rust code that represents
            // the damage of this ability
            "damage": [
                "60 + (0.6 * ctx.ap)",
                "110 + (0.6 * ctx.ap)",
                "160 + (0.6 * ctx.ap)",
                "210 + (0.6 * ctx.ap)",
                "260 + (0.6 * ctx.ap)"
            ]
        }
    ],
    // ...
  ],
  // ...
}
```

Note that if the function runs successfully, the damage formula of all abilities is recovered. To call some generator specifically, you can do:

```rs
ChampionFactory::run(ChampionId::Neeko);
let data = std::fs::read("internal/champions/Neeko.json")?;
let neeko_data = serde_json::from_slice::<Champion>(&data)?;
let neeko_q1 = neeko_data.abilities[0];

assert_eq!(neeko_q1.0, Q::_1);
assert_eq!(neeko_q1.1.damage[4], "260 + (0.6 * ctx.ap)");

// to run all generators
ChampionFactory::run_all()?;

// to run item generators
ItemFactory::run_all()?;
```

Also, this module scrapes from the internet the usual combos for every champion, and their recommended items/runes for each position (top, mid, jungle, adc, support). Check folder `internal/scraper/data.json`

### Example of scraped data

```jsonc
"Aurora": {
  "jungle": [
    [
      "LiandrysTorment", "Malignance",
      "VoidStaff", "Cryptbloom",
      "Riftmaker"
    ],
    [
      "TasteOfBlood", "GrislyMementos",
      "UltimateHunter", "ManaflowBand",
      "Transcendence", "AttackSpeed",
      "AdaptiveForce", "HealthScaling"
    ]
  ],
  // ...
}
```

### `tutorlolv2_fmt`

Exports functions that compresses data, converts literal code to html, format code, minifies html, or convert strings to PascalCase

- `rust_fmt(&str, usize)` Takes literal rust code as string and formats it and adds 4-space indentation
- `rust_html(&str)` Converts literal rust code as string and converts to HTML with appropriate classes that will look identical to the ones used by Visual Studio Code
- `json_html(&str)` same as `rust_html` function but with JSON strings
- `encode_brotli_11(&[u8])` Compresses data using brotli at the maximum compression level
- `encode_zstd_9` Compresses data using zstd library to the maximum compression level

### `tutorlolv2_server`

Same as a common ExpressJS http server, but using `actix_web`. Exposes routes that call functions from the core library, serve images, or use functions exported by the `dev` module.

```sh
POST http://localhost:8082/api/games/realtime
GET http://localhost:8082/img/champions/Neeko.avif
GET http://localhost:8082/api/setup/project
```

### `tutorlolv2_types`

Exports enums `AbilityId`, `AbilityName`, and `StatName`, which are used by almost all modules. There's nothing special about this crate

### `tutorlolv2_build`

Reads all data generated in the `internal` folder and generates a huge rust file at `tutorlolv2_gen/src/data.rs`. Generates structs, arrays and phf maps with the necessary data to evaluate damages of all kinds, as well as a big brotli-compressed block of about 6MB in size, and several offsets to it, which represents the internal source code used by this application. It is helpful to let users see how the damages are being calculated, if they're accurate, and most importantly, if they're outdated.

### Examples of generated structures (`tutorlolv2_gen`)

```rs
#[derive(...)]
#[repr(u8)]
pub enum ChampionId {
    Aatrox,
    Ahri,
    Akali,
    // ...
}

pub static CHAMPION_ID_TO_NAME: [&str; ChampionId::VARIANTS];
// To get the name of some champion, you can do:
let neeko = CHAMPION_ID_TO_NAME[ChampionId::Neeko];

pub static RECOMMENDED_ITEMS: [[&[ItemId]; 5]; 172];
// to get the recommended items of some champion, for
// a given position, you can do the following
RECOMMENDED_ITEMS[ChampionId::Neeko][Position::Middle];

pub static NEEKO: CachedChampion = CachedChampion {
    name: "Neeko",
    adaptative_type: AdaptativeType::Magic,
    attack_type: AttackType::Ranged,
    positions: &[Position::Middle, Position::Support],
    metadata: &[
        TypeMetadata {
            kind: AbilityId::Q(AbilityName::_1),
            damage_type: DamageType::Magic,
            attributes: Attrs::Undefined,
        },
        /// ...
    ],
    stats: CachedChampionStats {
        health: CachedChampionStatsMap {
            flat: 610f32,
            per_level: 104f32,
        },
        // ...
    },
    merge_data: &[],
    closures: &[
        neeko_q_1, neeko_q_2, neeko_q_3, 
        neeko_q_4, neeko_w_1, neeko_e_1,
        neeko_r_1,
    ],
};

// Example of generated const fn that represents
// the damage of some ability. Note that it must
// be present in `.closures` slice
pub const fn neeko_q_1(ctx: &Ctx) -> f32 {
    match ctx.q_level {
        1 => 60f32 + 0.6f32 * ctx.ap,
        2 => 110f32 + 0.6f32 * ctx.ap,
        3 => 160f32 + 0.6f32 * ctx.ap,
        4 => 210f32 + 0.6f32 * ctx.ap,
        5 => 260f32 + 0.6f32 * ctx.ap,
        _ => 0.0,
    }
}

// Example of generated rune
pub static ELECTROCUTE_8112: CachedRune = CachedRune {
    name: "Electrocute",
    damage_type: DamageType::Adaptative,
    metadata: TypeMetadata {
        kind: RuneId::Electrocute,
        damage_type: DamageType::Adaptative,
        attributes: Attrs::Undefined,
    },
    riot_id: 8112,
    internal_id: RuneId::Electrocute,
    undeclared: false,
    melee_damage: electrocute_melee,
    ranged_damage: electrocute_ranged,
};

// Several variables hold tuples of offsets that can index
// the constant [`BLOCK`]
pub static TOWER_DAMAGE_OFFSET: (u32, u32) = (6895847, 6897505);

// Avoid allocating more space than necessary
pub const BLOCK_SIZE: usize = 6898207;
// Compressed block. It has to be decompressed and casted
// to a &'static str type to recover some HTML
pub const BLOCK: [u8; 99177];

// To get the HTML that represents the formula used internally
// to calculate the damage against towers, you can do:
let bytes = decompress(&BLOCK, BLOCK_SIZE);
let block_str = unsafe { std::str::from_utf8_unchecked(bytes) };

let (i, j) = TOWER_DAMAGE_OFFSET;
let tower_damage_html = block_str[i..j];
```

The generated file has over 60,000 lines, and intense work is needed to generate it. For this reason, a recent rework was done in this module, and now all CPU cores are used to generate this file, now taking about 1 minute to finish the work.
- All tasks that can be done independently run in its own thread, and when it finishes, the dependent result is collected and parsed in the synchronously

### Frontend and Desktop Application

Check the following repositories
- [tutorlolv2_web](https://github.com/LuizGomes56/tutorlolv2_web)
- [tutorlolv2_desktop_app](https://github.com/LuizGomes56/tutorlolv2_desktop_app)

So far, all frontend prototypes are done using Web Assembly (WASM), with the framework `yew`, which is very similar to React. The language distribution in those project is of about:

- **HTML** (45%)
    - Written using the macro `html!` exported by yew
- **Rust** (35%)
    - Application logic and trait implementations
- **JavaScript** (10%)
    - Hover feature and page events related to it are written in JavaScript, sometimes directly referencing objects owned by the WASM application
- **CSS** (10%)
    - Most of the CSS is inlined in the HTML through `tailwind_css` classes

The expected size of the whole frontend application is 1.3MB (WASM), while the complete desktop application is expected to weigh 12MB, including the overlay viewer

## Key features

- Memory layout optimized to reduce total size of data generated while calculating data. 
    - `Realtime<'a>` size reduced from 195kB to 100kB
    - `InputGame` size reduced from 800B to 300B

        This action likely helped reducing cache misses

- Removed use of any `String` type, now only `Box<[T]>` is used. This way there's no need to convert an expression dynamically. Instead an static closure is generated, look at the example below

    ```rs
    // This involves string parsing
    let expr = "180 + 0.8 * AP"
    let damage = eval(expr);

    // This approach is best
    let damage_fn = |ctx| 180 + 0.8 * ctx.ap;
    let damage = damage_fn(Ctx::default());
    ```

- Almost all functions have a `const` equivalent

### Performance
- The average runtime of `realtime` function in the current [TypeScript backend](https://github.com/LuizGomes56/TSRemakeTutorLoL) is `18ms`. The new version runs in about `80μs`, while comparing the damages of 117 more items. Without considering it, the performance enhancement is 225x. If all items are considered, we can conclude it is approximately 20,000x faster

- Average memory consumption reduced from 400MB to 36MB

### Pending features / implementations

- <strong style="color: #81fe81ff">[Easy]</strong> Page design (HTML / CSS)
- <strong style="color: #81fe81ff">[Easy]</strong> Component `Calculator`, gathering inputs about the selected champion, its stats, owned items, runes, which enemies to consider and more.
- <strong style="color: #81fe81ff">[Easy]</strong> Component `Realtime` - Display damage calculations in a simplified way, as well as recovering data from port 2999
- <strong style="color: #81fe81ff">[Easy]</strong> Overlay feature - It should be similar to component `Realtime`, but with less data being displayed, and very small, placed in the corners of the screen, with the ability to use keyboard shortcuts to interact with the application
- <strong style="color: #81fe81ff">[Easy]</strong> Verify if the damage calculations used in this application are correct (check if the math adds up)
- <strong style="color: #fee181ff">[Medium]</strong> Include item/rune exceptions - Some items may add more stats under some conditions. Most of these exceptions are already implemented, but some are still missing
- <strong style="color: #fee181ff">[Medium]</strong> Add support for other game modes such as Arena and ARAM, adding the proper damage buffs or debuffs according to the current champion being played, as well as item buffs/debuffs. For example, in Arena mode, the Nashor's Tooth item may give more ability power than in the regular Summoner's Rift game mode
- <strong style="color: #fec481ff">[Medium]</strong> Generate better HTML files with the documentation of each item, champion and rune. In other words, improve its design and add more useful information. This is important to help indexation and increase the amount of content available under our domain
    - Check the [Formula Definitions](http://tutorlol.com/formulas) page from from the first version of `tutorlol` to see a prototype of how it should look like
- <strong style="color: #fe8181ff">[Hard]</strong> Complete generator function for all items and champions. Currently only `5/172` champions and `2/81` items have been marked `#![stable]`
- <strong style="color: #fe8181ff">[Hard]</strong> Create a new database and save the captured data from port 2999 sent by users so they can access it back in the future, as well as to create statistics about the usage of this app
- <strong style="color: #ff0000ff">[Very Hard]</strong> Implement a safer version of the low-level keyboard in `src-tauri` module
- <strong style="color: #ff0000ff">[Very Hard]</strong> Add FFI bindings to make the library available in other languages such as TypeScript and Python

## Running the project and Documentation

Download the project and run `cargo d` to get a detailed webpage explaining how to use each function exported by each internal library

To run the project, do the following:

> cd tutorlolv2_server

- Running server
    > cargo run -r

- Setting up the project (generating cache folders)
    > GET `http://localhost:8082/api/setup/project`

- Running build script (create `tutorlolv2_gen/src/data.rs`)
    > cd tutorlolv2_build

    > cargo run -r

To interact with the math module library, you can edit `src/main.rs` directly