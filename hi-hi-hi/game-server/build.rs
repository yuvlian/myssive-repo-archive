use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

const ROUTER_MACRO: &str = r#"use crate::handlers::*;
use database::Arclite;
use hi3_proto::cmd::*;
use paste::paste;

macro_rules! handle {
    ($($handler:ident);* $(;)?) => {
        paste! {
            pub async fn ping_pong((cmd_id, user_id, body): (u32, u32, &[u8]), db: &Arclite) -> Vec<u8> {
                match cmd_id {
                    $(
                        [<$handler:upper _REQ>] => $handler::handle(user_id, body, db).await,
                    )*
                    _ => Vec::with_capacity(0),
                }
            }
        }
    };
}"#;

fn main() {
    let handlers_dir = "./src/handlers/";
    let mod_rs_path = "./src/handlers/mod.rs";
    let router_rs_path = "./src/network/router.rs";

    let mut mod_rs = File::create(mod_rs_path).unwrap();
    let mut router_rs = File::create(router_rs_path).unwrap();

    writeln!(router_rs, "{}", ROUTER_MACRO).unwrap();

    let mut handlers = Vec::new();

    for entry in WalkDir::new(handlers_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path().extension().map(|ext| ext == "rs").unwrap_or(false)
                && e.file_name() != "mod.rs"
        })
    {
        let file_name = entry.file_name().to_string_lossy();
        let module_name = file_name.trim_end_matches(".rs");
        handlers.push(module_name.to_string());
        writeln!(mod_rs, "pub mod {};", module_name).unwrap();
    }

    writeln!(router_rs, "handle![").unwrap();
    for handler in &handlers {
        writeln!(router_rs, "    {};", handler).unwrap();
    }
    writeln!(router_rs, "];").unwrap();
}
