use tutorlolv2_build_dep::{BuildContext, MayFail, project_root};

fn main() -> MayFail {
    let mut context = BuildContext::new(project_root(), "build_output", true)?;

    context.export_json = true;

    let mut args = std::env::args_os().skip(1);

    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--out-dir") => {
                let out = args.next().ok_or("--out-dir requires a path")?;
                context.out = context.root.join(out);
            }
            Some("--no-docs") => context.docs = false,
            Some("--no-export") => context.export_json = false,
            Some("--help" | "-h") => {
                println!("Usage: tutorlolv2_build_dep [--out-dir PATH] [--no-docs] [--no-export]");
                return Ok(());
            }
            _ => return Err(format!("Unknown argument: {arg:?}").into()),
        }
    }

    context.run()
}
