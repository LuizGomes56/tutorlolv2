use crate::{
    JsonWrite, MayFail,
    client::{SaveTo, Tag},
};
use serde::Serialize;
use tutorlolv2_fmt::rustfmt;

pub mod fac_items;

pub mod wiki_champions;
pub mod wiki_runes;

pub trait Parser<T: Serialize>
where
    Self: Sized,
{
    const TAG: Tag;

    fn new() -> MayFail<Self>;
    fn keys(&self) -> Vec<&str>;
    fn run_fn(&self, id: &str) -> MayFail<T>;
    fn create_methods(&self, result: &mut String, id: &str);

    fn run_all(self) -> MayFail {
        for key in self.keys() {
            self.run(key)?
        }
        Ok(())
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
        for name in self.keys() {
            if let Ok(data) = std::fs::read_to_string(SaveTo::GeneratorRaw(Self::TAG, name).path())
            {
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
        if !self.keys().contains(&id) {
            return Err(format!("[WikiFactory::create] {id} not found").into());
        }

        if let Ok(text) = std::fs::read_to_string(SaveTo::GeneratorRaw(Self::TAG, id).path())
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

        self.create_methods(&mut result, id);

        result.push_str(".end()}}");

        let formatted = rustfmt(&result, None);
        let content = match formatted.is_empty() {
            true => result,
            false => formatted,
        };

        let path = SaveTo::GeneratorRaw(Self::TAG, id).path();

        println!("[write] {path:?}");
        std::fs::write(&path, content)?;

        Ok(())
    }

    fn create_all(&self) -> MayFail {
        self.keys().iter().try_for_each(|key| self.create(key))
    }
}
