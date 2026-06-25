use tutorlolv2_build_dep::{CPARSER, IPARSER, MayFail, RPARSER, generators::parser::Parser, run};

fn main() -> MayFail {
    std::env::set_current_dir("..")?;

    println!("Champions");
    CPARSER.progress();
    println!("\nItems");
    IPARSER.progress();
    println!("\nRunes");
    RPARSER.progress();

    CPARSER.run_all();
    IPARSER.run_all();
    RPARSER.run_all();

    run()
}
