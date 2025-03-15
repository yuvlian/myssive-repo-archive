use std::fs;

fn main() {
    let handlers_path = "/home/yulian/dls/ghub/ultina-rs/ultina-rs/src/handlers";

    println!("cargo:rerun-if-changed={}", handlers_path);

    let mod_rs_path = format!("{}/mod.rs", handlers_path);

    let mut mods = Vec::new();

    if let Ok(h) = fs::read_dir(handlers_path) {
        for v in h.flatten() {
            let path = v.path();
            if let Some(ex) = path.extension() {
                if ex == "rs" {
                    if let Some(s) = path.file_stem() {
                        if let Some(n) = s.to_str() {
                            if n != "mod" {
                                mods.push(n.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut mod_rs = String::new();

    for m in &mods {
        mod_rs.push_str(&format!("pub mod {};\n", m));
    }

    fs::write(mod_rs_path, mod_rs).expect("Failed to write mod.rs");
}
