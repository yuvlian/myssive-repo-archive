use heck::ToShoutySnakeCase;
use regex::Regex;
use std::fs::{File, create_dir_all};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::sync::LazyLock;

static MESSAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s*message\s+(\w+)\s*\{").unwrap());

static CMD_ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bCMD_ID\s*=\s*(\d+);").unwrap());

// Prost output file is _.rs
const OUTPUT_DIR: &str = "out/";
const CMD_OUT: &str = "./out/cmd.rs";
// Source file
const PROTO_FILE: &str = "proto/raw/BH3_8.1.12.proto";
// Used to alloc a vec, not necessary but why not lol
const PROTO_FILE_LINE_COUNT: usize = 67139;

fn main() -> io::Result<()> {
    if !Path::new(OUTPUT_DIR).exists() {
        create_dir_all(OUTPUT_DIR)?;
    }

    if Path::new(PROTO_FILE).exists() {
        println!("cargo::rerun-if-changed={}", PROTO_FILE);

        prost_build::Config::new()
            .out_dir(OUTPUT_DIR)
            .compile_protos(&[PROTO_FILE], &["."])
            .expect("Failed to compile protobuf");

        let cmd_output = parse_cmd_ids(PROTO_FILE)?;

        let mut file = File::create(CMD_OUT)?;

        for line in cmd_output {
            writeln!(file, "{}", line)?;
        }
    } else {
        panic!(
            "`{}` does not exist\ngo grab it from `git.neonteam.dev/amizing/BH3-Proto`",
            PROTO_FILE
        );
    }

    Ok(())
}

fn parse_cmd_ids(file_name: &str) -> io::Result<Vec<String>> {
    let mut cmd_file: Vec<(u32, String)> = Vec::with_capacity(PROTO_FILE_LINE_COUNT / 20usize);
    let mut current_message_name: Option<String> = None;

    let file = File::open(file_name)?;
    let lines = io::BufReader::new(&file).lines();

    for line in lines {
        let line = line?;

        if let Some(captures) = MESSAGE_RE.captures(&line) {
            let raw_message_name = &captures[1];
            current_message_name = Some(raw_message_name.to_shouty_snake_case());
            continue;
        }

        if let Some(message_name) = &current_message_name {
            if let Some(captures) = CMD_ID_RE.captures(&line) {
                let cmd_id_number: u32 = captures[1].parse().unwrap();
                cmd_file.push((cmd_id_number, message_name.clone()));
            }
        }
    }

    cmd_file.sort_by_key(|x| x.0);

    let result = cmd_file
        .into_iter()
        .map(|(cmd_id, message_name)| format!("pub const {}: u32 = {};", message_name, cmd_id))
        .collect();

    Ok(result)
}
