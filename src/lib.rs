#![allow(ambiguous_glob_reexports)]
#[cfg(feature = "dev")]
pub use tutorlolv2_dev::*;
pub use tutorlolv2_gen::*;
pub use tutorlolv2_math::*;

#[cfg(test)]
mod tests {
    use std::thread::spawn;
    use tutorlolv2_exports::*;

    #[test]
    fn generate_html() {
        let champions = spawn(generate_champion_html);
        let items = spawn(generate_item_html);
        let runes = spawn(generate_rune_html);

        champions.join().unwrap();
        items.join().unwrap();
        runes.join().unwrap();
    }

    #[test]
    fn __update() {
        // cargo run -r
        // Wait compilation, then wait 30 seconds for server to start
        // call `localhost:8082/api/setup/project`
        // wait conclusion by monitoring stdio until no message is sent for more than 30 seconds
        // call `localhost:8082/api/images/compress` if necessary
        // when compression is done and do `cd tutorlolv2_build && cargo run -r`
        // wait process completion (when it exits)
        // call `localhost:8082/api/setup/docs`
    }
}
