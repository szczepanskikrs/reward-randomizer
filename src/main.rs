use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "This program provides random rewards list from configured rewards.json file after you finish your learning session"
)]
struct Args {
    #[arg(short, long)]
    max_count: Option<usize>,
}
#[derive(Serialize, Deserialize, Debug)]
struct RewardsWrapper {
    rewards: Vec<String>,
}

enum OsDefaultLocation {
    Linux { default_file: String },
    Mac { default_file: String },
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::rng();
    let mut home_directory =
        dirs::home_dir().expect("Failed to resolve home directory is $HOME env variable set?");

    let os_specific_file = match extract_os_specific_rewards_file() {
        Ok(system_default) => match system_default {
            OsDefaultLocation::Linux { default_file } => default_file,
            OsDefaultLocation::Mac { default_file } => default_file,
        },
        Err(panic_message) => panic!("{}", panic_message),
    };

    home_directory.push(os_specific_file);

    let home_directory_value = home_directory.to_str().unwrap_or("unknown");

    let reward_count = rng.random_range(
        0..args
            .max_count
            .filter(|count| count >= &0usize)
            .unwrap_or_else(|| DEFAULT_MAX_REWARDS_SIZE),
    );

    let rewards_file = File::open(&home_directory).expect(
        format!(
            "File not found: {}. You need to setup rewards.json first",
            home_directory_value
        )
        .as_str(),
    );

    let rewards_wrapper: RewardsWrapper =
        serde_json::from_reader(rewards_file).expect("Error parsing rewards.json");

    let mut selected_rewards: Vec<&String> = vec![];

    for _ in 0..reward_count {
        let random_idx = rng.random_range(0..rewards_wrapper.rewards.len());
        selected_rewards.push(rewards_wrapper.rewards.get(random_idx).unwrap())
    }

    let rewards_list = selected_rewards
        .iter()
        .map(|e| e.as_str())
        .take(reward_count)
        .collect::<Vec<&str>>()
        .join(", ");

    match selected_rewards.len() {
        0 => println!("No rewards for you today!"),
        1 => println!("Selected reward [ {} ]", rewards_list),
        _ => println!("Selected rewards [ {} ]", rewards_list),
    }
}

fn extract_os_specific_rewards_file() -> Result<OsDefaultLocation, String> {
    match std::env::consts::OS {
        "linux" => Ok(OsDefaultLocation::Linux {
            default_file: DEFAULT_LOCATION_LINUX.to_string(),
        }),
        "mac" => Ok(OsDefaultLocation::Mac {
            default_file: DEFAULT_LOCATION_MAC.to_string(),
        }),
        unsupported => Err(format!("System: {} is not supported!", unsupported)),
    }
}

const DEFAULT_MAX_REWARDS_SIZE: usize = 2;
const DEFAULT_LOCATION_LINUX: &str = ".config/reward-randomizer/rewards.json";
const DEFAULT_LOCATION_MAC: &str = "Library/Application Support/reward-randomizer/rewards.json";
