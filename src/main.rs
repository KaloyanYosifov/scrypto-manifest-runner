use std::{collections::HashMap, fs, path::Path, process::Command};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    manifest: String,

    #[arg(short, long)]
    arguments: Vec<String>,
}

type ManifestArguments = HashMap<String, String>;

fn replace_variables(mut data: String, arguments: ManifestArguments) -> String {
    for (key, value) in arguments {
        let key_to_replace = format!("${{{}}}", &key);
        data = data.replace(&key_to_replace, &value);
    }

    data
}

fn parse_arguments(arguments: Vec<String>) -> ManifestArguments {
    let mut map = HashMap::new();

    for argument in arguments {
        let mut parts = argument.split("=");
        let key = match parts.next() {
            Some(key) => key,
            _ => panic!("Failed to parse argument!"),
        };
        let value = match parts.next() {
            Some(value) => value,
            _ => panic!("Failed to parse argument!"),
        };

        map.insert(key.to_string(), value.to_string());
    }

    map
}

fn main() {
    let args = Args::parse();
    let manifest_path = Path::new(&args.manifest);

    if !manifest_path.exists() {
        panic!("Cannot find manifest file!");
    }

    match fs::read_to_string(manifest_path) {
        Ok(data) => {
            let parsed_arguments = parse_arguments(args.arguments);
            let replaced = replace_variables(data, parsed_arguments);

            // TODO: store into a temp file

            // call with temp file
            let output = Command::new("resim")
                .arg("run")
                .arg(args.manifest)
                .output()
                .expect("Failed to execute command");

            // print output and error
            print!("{}", String::from_utf8_lossy(&output.stdout));
            print!("{}", String::from_utf8_lossy(&output.stderr));

            // delete temp file
        }
        _ => panic!("Lol"),
    }
}
