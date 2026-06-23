use tutorlolv2_build_dep::MayFail;

fn main() -> MayFail {
    std::env::set_current_dir("..")?;
    tutorlolv2_build_dep::run()
}
