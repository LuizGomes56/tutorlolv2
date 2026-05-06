use crate::{
    GeneratorExt, JsonWrite, MayFail,
    client::{SaveTo, Tag},
};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, path::Path};
use tutorlolv2_fmt::rustfmt;

pub mod wiki_champions;
pub mod wiki_items;
pub mod wiki_runes;

pub trait Parser<T, U>
where
    Self: Sized + Sync,
    T: Clone + DeserializeOwned + Send + Sync + 'static,
    U: Serialize,
{
    const TAG: Tag;
    const FN: fn(&str) -> Option<fn(T) -> Box<dyn GeneratorExt<U>>>;

    fn new() -> MayFail<Self>;
    fn map(&self) -> &BTreeMap<String, T>;
    fn create_methods(&self, result: &mut String, id: &str) -> bool;

    fn run_fn(&self, id: &str) -> MayFail<U> {
        self.map()
            .get(id)
            .and_then(|data| {
                let function = Self::FN(id)?;
                Some(function(data.clone()))
            })
            .ok_or_else(|| format!("[WikiFactory::run] {id} not found"))?
            .call()
    }

    fn run_all(&self) -> MayFail {
        self.map()
            .keys()
            .par_bridge()
            .try_for_each(|key| self.run(key))
    }

    fn run(&self, id: &str) -> MayFail {
        match self.run_fn(id) {
            Ok(value) => value.into_file(SaveTo::InternalRaw(Self::TAG, id).path()),
            Err(e) => Ok(println!("Error generating {id:?}: {e:?}")),
        }
    }

    fn progress(&self) {
        let mut stables = 0;
        let mut preserve = 0;
        let mut unstables = 0;
        let mut total = 0;
        for name in self.map().keys() {
            if let Ok(data) = crate::read_to_string(SaveTo::GeneratorRaw(Self::TAG, name).path()) {
                if data.contains("Stable") {
                    stables += 1;
                } else if data.contains("Preserve") {
                    preserve += 1;
                } else {
                    unstables += 1;
                }
                total += 1;
            }
        }

        println!(
            concat!(
                "Parser::progress\n",
                "{stables:>3} / {total} stable\n",
                "{preserve:>3} / {total} preserved\n",
                "{unstables:>3} / {total} unstable\n",
            ),
            stables = stables,
            preserve = preserve,
            unstables = unstables,
            total = total
        );
    }

    fn create(&self, id: &str) -> MayFail {
        if !self.map().keys().any(|k| k == id) {
            return Err(format!("[WikiFactory::create] {id} not found").into());
        }

        if let Ok(text) = crate::read_to_string(SaveTo::GeneratorRaw(Self::TAG, id).path())
            && (text.contains(".progress(Stable)") || text.contains(".progress(Preserve)"))
        {
            println!("[stable] Skipping generator for {id:?}");
            return Ok(());
        }

        let mut result = format!(
            "use super::*;

            impl Generator for {id} {{
                fn generate(&mut self) -> MayFail {{ self"
        );

        let path = SaveTo::GeneratorRaw(Self::TAG, id).path();
        let dir = SaveTo::GeneratorDir(Self::TAG).path();

        crate::create_dir_all(dir)?;

        match self.create_methods(&mut result, id) {
            true => {
                result.push_str(".end()}}");

                let formatted = rustfmt(&result, None);
                let content = match formatted.is_empty() {
                    true => result,
                    false => formatted,
                };

                crate::write(&path, content)
            }
            false => Ok(crate::remove_file(&path)),
        }
    }

    fn create_all(&self) -> MayFail {
        let keys = self.map().keys().map(String::as_str).collect::<Vec<_>>();
        let tag = Self::TAG;

        let dir_loc = SaveTo::GeneratorDir(tag).path();
        let dir = Path::new(&dir_loc);
        crate::create_dir_all(dir)?;

        let decl = dir.join("mod").with_extension("rs");
        let module = format_args!("decl_{tag}");

        keys.par_iter().try_for_each(|key| self.create(key))?;

        let modules = keys
            .iter()
            .copied()
            .filter_map(|key| {
                let loc = SaveTo::GeneratorRaw(Self::TAG, key).path();
                Path::new(&loc).exists().then_some(key)
            })
            .collect::<Vec<_>>()
            .join(",\n\t");

        let decl_content = format!("use super::{module}::*;\ncrate::{module}!(\n\t{modules}\n);",);

        crate::write(&decl, decl_content)?;

        Ok(())
    }

    fn infer_damage_type(result: &mut String, description: &str) {
        if let Some(dtype) = ["physical", "physical", "true"]
            .into_iter()
            .find(|d| description.to_lowercase().contains(d))
        {
            let alias = dtype
                .chars()
                .next()
                .map(|c| c.to_uppercase().collect::<String>() + &dtype[1..])
                .unwrap_or(dtype.to_string());

            result.push_str(&format!(".damage_type({alias})"));
        }
    }
}
