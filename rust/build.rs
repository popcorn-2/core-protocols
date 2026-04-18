use std::env;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt::Write;

fn main() -> io::Result<()> {
    let src_dir = {
        let mut buf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        buf.push("../src");
        buf
    };

    let out = visit_dir(&src_dir, Path::new(""))?;
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("server.gen.rs"), out)?;

    Ok(())
}

fn visit_dir(root: &Path, extra: &Path) -> io::Result<String> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("server");

    let dir = {
        let mut buf = PathBuf::from(root);
        buf.push(extra);
        buf
    };

    let mut buf = String::new();

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        if ty.is_dir() {
            let subdir = extra.join(entry.file_name());
            let _ = writeln!(&mut buf, "pub mod {} {{", entry.file_name().display());
            buf += &visit_dir(root, &subdir)?;
            buf += "}\n\n";
        } else if ty.is_file() {
            let dir = out_dir.join(&extra);
            println!("cargo::rerun-if-changed={}", path.display());
            fs::create_dir_all(&dir)?;
            pipc::process_file(&path, pipc::OutputFormat::Rust, pipc::OutputTy::Server, &dir)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let _ = writeln!(&mut buf, "#[path = \"{1}/{0}.rs\"] pub mod {0};", path.file_stem().unwrap().display(), dir.display());
        }
    }

    Ok(buf)
}