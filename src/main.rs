use rand::prelude::*;
use std::{collections::HashMap, env, fs, path::Path, process::Command};

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

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

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

    arguments.iter().for_each(|argument| {
        let mut parts = argument.split("=");

        match (parts.next(), parts.next()) {
            (Some(key), Some(value)) => map.insert(key.to_string(), value.to_string()),
            _ => panic!("Failed to parse argument!"),
        };
    });

    map
}

fn within_temp_dir(
    callable: &dyn Fn(&str) -> Result<(), std::io::Error>,
) -> Result<(), std::io::Error> {
    let mut path = env::current_dir()?;

    path.push("___tmp_manifests___");

    if !path.exists() {
        fs::create_dir(&path)?;
    }

    callable(path.to_str().unwrap())?;

    fs::remove_dir_all(path)?;

    Ok(())
}

fn generate_random_file_id(len: i32) -> String {
    let mut test: String = "".to_string();
    let mut rng = rand::thread_rng();

    for _i in 0..len {
        let index: usize = rng.gen_range(0..ALPHABET.chars().count());

        test.push(ALPHABET.chars().nth(index).unwrap());
    }

    return test;
}

fn main() {
    let args = Args::parse();
    let manifest_path = Path::new(&args.manifest);

    if !manifest_path.exists() {
        panic!("Cannot find manifest file!");
    }

    let data =
        fs::read_to_string(manifest_path).unwrap_or_else(|_| panic!("Failed to read manifest!"));

    let parsed_arguments = parse_arguments(args.arguments);
    let replaced = replace_variables(data, parsed_arguments);

    within_temp_dir(&|path: &str| {
        let transaction_file = format!(
            "{}/{}",
            path,
            format!("{}.rtm", generate_random_file_id(10))
        );

        fs::write(&transaction_file, &replaced)?;

        let output = Command::new("resim")
            .arg("run")
            .arg(transaction_file)
            .output()
            .expect("Failed to execute command");

        // print output and error
        print!("{}", String::from_utf8_lossy(&output.stdout));
        print!("{}", String::from_utf8_lossy(&output.stderr));

        Ok(())
    })
    .unwrap();
}
