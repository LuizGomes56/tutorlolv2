use crate::scripts::{
    BASIC_ATTACK, BASIC_ATTACK_FN, CRITICAL_STRIKE, CRITICAL_STRIKE_FN, DEFAULT_ITEM_GENERATOR,
    IGNITE_FN, ONHIT_EFFECT, ONHIT_EFFECT_FN, TOWER_DAMAGE, TOWER_DAMAGE_FN, ZERO_FN,
    batch::{FmtOutput, batch},
    champions::{self, generate_champions},
    items::generate_items,
    runes::{self, generate_runes},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::BTreeMap, ops::Range};
use tutorlolv2_dev::MayFail;
use tutorlolv2_fmt::encode_brotli_11;

mod scripts;

/// Provides functions to help track the current and new offsets of some
/// data inside a very large string
pub struct Tracker<'a> {
    inner: &'a mut String,
}

impl<'a> Tracker<'a> {
    /// Creates a new instance of self, from an existing string that
    /// should live longer than this struct
    pub const fn new(inner: &'a mut String) -> Self {
        Self { inner }
    }

    /// Get the current length of the string, which represents
    /// the `end` offset of the last record
    pub const fn offset(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, value: &str) -> Range<usize> {
        let start = self.offset();
        self.inner.push_str(value);
        start..self.offset()
    }

    pub fn batch(&mut self, batch: &mut BTreeMap<&str, BTreeMap<&str, Vec<FmtOutput<'_>>>>) {
        for value in batch.values_mut() {
            for data in value.values_mut() {
                for output in data.iter_mut() {
                    if !output.json.default {
                        output.html_range = self.push(&output.html);
                    }
                }
            }
        }
    }
}

pub static mut ZERO_FN_OFFSET: Range<usize> = 0..0;
pub static mut DEFAULT_ITEM_GENERATOR_OFFSET: Range<usize> = 0..0;

/// Entry point of the build library. Generates a new library that will be
/// used by both frontend and backend, as well as HTML that represents the
/// internal code that will be shown when hovering over some objects in the
/// frontend application
pub fn run() -> MayFail {
    let mut full_block = String::with_capacity(12 * 1024 * 1024);
    let mut exports = String::with_capacity(4 * 1024 * 1024);

    exports.push_str("use super::*;");

    let mut tracker = Tracker::new(&mut full_block);

    let closures = [generate_champions, generate_items, generate_runes]
        .into_par_iter()
        .map(|task| task().unwrap())
        .collect::<Vec<_>>();

    unsafe {
        DEFAULT_ITEM_GENERATOR_OFFSET =
            tracker.push(&tutorlolv2_fmt::rust_html(DEFAULT_ITEM_GENERATOR));
        ZERO_FN_OFFSET = tracker.push(&tutorlolv2_fmt::rust_html(ZERO_FN));
    }

    println!("[ok] Generation task finished. Processing results");

    for (name, value) in [
        ("IGNITE_OFFSET", IGNITE_FN),
        ("ONHIT_EFFECT_OFFSET", ONHIT_EFFECT),
        ("BASIC_ATTACK_OFFSET", BASIC_ATTACK),
        ("TOWER_DAMAGE_OFFSET", TOWER_DAMAGE),
        ("CRITICAL_STRIKE_OFFSET", CRITICAL_STRIKE),
        ("ONHIT_EFFECT_FN_OFFSET", ONHIT_EFFECT_FN),
        ("TOWER_DAMAGE_FN_OFFSET", TOWER_DAMAGE_FN),
        ("BASIC_ATTACK_FN_OFFSET", BASIC_ATTACK_FN),
        ("CRITICAL_STRIKE_FN_OFFSET", CRITICAL_STRIKE_FN),
    ] {
        let range = tracker.push(&&tutorlolv2_fmt::rust_html(value));
        exports.push_str(&format!("pub static {name}: Range<usize> = {range:?};"));
    }

    for ((function, finish), module) in closures
        .into_iter()
        .zip([champions::finish, runes::finish, runes::finish])
        .zip(["champions", "items", "runes"])
    {
        let (mut fmt_args, fmt) = function;
        let mut src = tutorlolv2_fmt::rustfmt(&fmt, None);
        let mut batch = batch(&src);
        tracker.batch(&mut batch);

        let mut delete_ranges = batch
            .into_values()
            .map(|output| {
                output
                    .into_iter()
                    .map(|(target, value)| {
                        let variable = fmt_args.get_mut(target).unwrap();

                        let deletes = value
                            .iter()
                            .map(|output| output.delete_range.clone())
                            .collect::<Vec<_>>();

                        finish(target, variable, value);

                        deletes
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .flatten()
            .collect::<Vec<_>>();

        delete_ranges.sort_by_key(|range| range.start);

        for range in delete_ranges.into_iter().rev() {
            src.drain(range);
        }

        let mut block = fmt_args
            .values_mut()
            .map(|variable| {
                variable.push_str("];");
                variable.as_str()
            })
            .collect::<String>()
            + &src;

        block.insert_str(0, "use super::*;\n");

        let fmt_block = tutorlolv2_fmt::rustfmt(&block, None);

        tutorlolv2_dev::write(
            format!("tutorlolv2_gen/src/generated/{module}.rs"),
            fmt_block,
        )?;
    }

    println!("[ok] Formatting generated file");

    let final_exports = tutorlolv2_fmt::rustfmt(&exports, None);

    println!("[ok] Writing exports and block");

    tutorlolv2_dev::write("tutorlolv2_gen/src/generated/exports.rs", &final_exports)?;
    tutorlolv2_dev::write("tutorlolv2_gen/src/block.txt", &full_block)?;

    println!("[ok] Compressing full block");

    let compressed_block = encode_brotli_11(full_block.as_bytes());

    println!("[ok] Saving brotli file");

    tutorlolv2_dev::write("tutorlolv2_gen/src/block.br", compressed_block)?;

    Ok(())
}
