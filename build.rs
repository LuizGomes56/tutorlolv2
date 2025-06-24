#[path = "build/phf_champions.rs"]
mod phf_champions;
#[path = "build/phf_items.rs"]
mod phf_items;
#[path = "build/phf_meta.rs"]
mod phf_meta;
#[path = "build/phf_names.rs"]
mod phf_names;
#[path = "build/phf_runes.rs"]
mod phf_runes;
#[path = "build/wr_formulas.rs"]
mod writers_and_formulas;

use crate::{
    phf_champions::global_phf_internal_champions, phf_items::global_phf_internal_items,
    phf_meta::global_phf_internal_meta_items, phf_names::global_phf_internal_names,
    phf_runes::global_phf_internal_runes, writers_and_formulas::writers_and_formulas,
};
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    writers_and_formulas(&out_dir);
    global_phf_internal_champions(&out_dir);
    global_phf_internal_items(&out_dir);
    global_phf_internal_runes(&out_dir);
    global_phf_internal_meta_items(&out_dir);
    global_phf_internal_names(&out_dir);
}
