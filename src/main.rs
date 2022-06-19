use clap::{Parser, Subcommand};
use jyt::{error, Converter, Ext};
use std::io;
use std::io::BufRead;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert Json to Yaml (also as `json2yaml`, `j2y`, and `jy`)
    #[clap(alias = "json2yaml", alias = "j2y", alias = "jy")]
    JsonToYaml { input: Option<String> },
    /// Convert Json to Toml (also as `json2toml`, `j2t`, and `jt`)
    #[clap(alias = "json2toml", alias = "j2t", alias = "jt")]
    JsonToToml { input: Option<String> },

    /// Convert Yaml to Json (also as `yaml2json`, `y2j`, and `yj`)
    #[clap(alias = "yaml2json", alias = "y2j", alias = "yj")]
    YamlToJson { input: Option<String> },
    /// Convert Yaml to Toml (also as `yaml2toml`, `y2t`, and `yt`)
    #[clap(alias = "yaml2toml", alias = "y2t", alias = "yt")]
    YamlToToml { input: Option<String> },

    /// Convert Toml to Json (also as `toml2json`, `t2j`, and `tj`)
    #[clap(alias = "toml2json", alias = "t2j", alias = "tj")]
    TomlToJson { input: Option<String> },
    /// Convert Toml to Yaml (also as `toml2yaml`, `t2y`, and `ty`)
    #[clap(alias = "toml2yaml", alias = "t2y", alias = "ty")]
    TomlToYaml { input: Option<String> },
}

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

fn input_from_stdin() -> String {
    // Read texts from stdin (including pipe)
    io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .join(LINE_ENDING)
}

fn extract_input(input: &Option<String>) -> String {
    if let Some(input) = input {
        input.clone()
    } else {
        input_from_stdin()
    }
}

fn main() -> Result<(), error::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::JsonToYaml { input } => {
            println!("{}", extract_input(input).to_yaml(Ext::Json)?.trim_end());
        }
        Commands::JsonToToml { input } => {
            println!("{}", extract_input(input).to_toml(Ext::Json)?.trim_end());
        }

        Commands::YamlToJson { input } => {
            println!("{}", extract_input(input).to_json(Ext::Yaml)?);
        }
        Commands::YamlToToml { input } => {
            println!("{}", extract_input(input).to_toml(Ext::Yaml)?.trim_end());
        }

        Commands::TomlToJson { input } => {
            println!("{}", extract_input(input).to_json(Ext::Toml)?);
        }
        Commands::TomlToYaml { input } => {
            println!("{}", extract_input(input).to_yaml(Ext::Toml)?.trim_end());
        }
    }
    Ok(())
}
