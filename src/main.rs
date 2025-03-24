use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{io, vec};

use clap::{Args, Parser, Subcommand, ValueEnum};
use response::key::KeyResponse;
use response::keys::CharacterProfile;
use serde::Deserialize;

mod response;

#[derive(Parser, Debug)]
#[clap(author, version, about = "WoW CLI for Raider.io API", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Keys(KeysCmd),
    Key(KeyCmd),
}

#[derive(Args, Debug)]
struct KeysCmd {
    #[clap(short, long)]
    pub character_name: String,
    #[clap(short, long)]
    pub region: Region,
    #[clap(short, long)]
    pub server: Server,
}

#[derive(Args, Debug)]
struct KeyCmd {
    #[clap(short, long)]
    pub id: usize,
}

#[derive(Debug, ValueEnum, Clone)]
enum Region {
    Us,
    Eu,
    Tw,
    Kr,
    Cn,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::Us => write!(f, "us"),
            Region::Eu => write!(f, "eu"),
            Region::Tw => write!(f, "tw"),
            Region::Kr => write!(f, "kr"),
            Region::Cn => write!(f, "cn"),
        }
    }
}

#[derive(Debug, ValueEnum, Clone)]
enum Server {
    ZulJin,
    Sargeras,
    Illidan,
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::ZulJin => write!(f, "zuljin"),
            Server::Sargeras => write!(f, "sargeras"),
            Server::Illidan => write!(f, "illidan"),
        }
    }
}

const BASE_URL: &str = "https://raider.io/api/v1";

#[derive(Debug)]
enum Error {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Io(io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

// Exercises:
//
// 1. Add new command to get a single key run and its details
//    - The run needs to be fetched by ID
//    - The stdout ouput must be structured the following way:
// filter the roles from tank->healer->dps and dynamically manage the +'s
// | role | ilvl | name |
// +10 - Grim Batol
//
// tank\t591\tTacostruck
// healer\t622\tGhostbrew
// dps\t632\tSpikyy
// dps\t628\tLanivar
// dps\t494\tJardom
//
//
// 2. Implement a cache for the key run details to avoid refetching the same data that cannot
//    change
//    - The cache should be keyed by the run ID
//    - The cache should be stored in a file
//

async fn get_rio<'a, T>(url: &'a String) -> Result<T, Error>
where
    T: for<'b> Deserialize<'b>,
{
    let res = reqwest::get(url).await?;

    let status = res.status();
    let text = res.text().await?;

    if !status.is_success() {
        println!("{}", text);
    }
    let res: T = serde_json::from_str(&text)?;
    Ok(res)
}

fn sort_roster(roster: &mut Vec<response::key::Roster>) {
    roster.sort_by_key(|roster| match roster.role.as_str() {
        "tank" => 0,
        "healer" => 1,
        _ => 2,
    });
}
#[derive(serde::Serialize)]
struct CachedKey {
    id: i64,
    num_chests: i64,
    level: i64,
    dungeon_name: String,
    roster: Vec<KeyMember>,
}
#[derive(serde::Serialize)]
struct KeyMember {
    role: String,
    item_level: i64,
    character_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    println!("{:?}", cli);

    match &cli.command {
        Commands::Keys(args) => {
            let url = format!(
                "{}/characters/profile?region={}&name={}&realm={}&fields=mythic_plus_recent_runs",
                BASE_URL, args.region, args.character_name, args.server
            );

            let json: CharacterProfile = get_rio(&url).await?;

            println!("Recent keys for {}\n", json.name);

            for key in &json.mythic_plus_recent_runs {
                println!(
                    "ID: {}\t+{} - {}",
                    key.keystone_run_id, key.mythic_level, key.dungeon
                );
                println!("------------------------------------");
            }
        }
        Commands::Key(args) => {
            let url = format!(
                "{}/mythic-plus/run-details?season=season-tww-2&id={}",
                BASE_URL, args.id
            );

            let mut json: KeyResponse = get_rio(&url).await?;

            let num_chests: String = (0..json.num_chests).map(|_| "+").collect();

            println!(
                "{}{} - {}\n",
                num_chests, json.mythic_level, json.dungeon.name
            );

            sort_roster(&mut json.roster);

            for member in &json.roster {
                println!(
                    "{}\t{}\t{}",
                    member.role, member.items.item_level_equipped, member.character.name
                );
            }

            let mut sorted_key_roster: Vec<KeyMember> = Vec::with_capacity(json.roster.len());

            for index in 0..json.roster.len() {
                let new_member = KeyMember {
                    role: json.roster[index].role.clone(),
                    item_level: json.roster[index].items.item_level_equipped.clone(),
                    character_name: json.roster[index].character.name.clone(),
                };
                sorted_key_roster.push(new_member);
            }

            let cached_key = CachedKey {
                id: json.keystone_run_id,
                num_chests: json.num_chests,
                level: json.mythic_level,
                dungeon_name: json.dungeon.name,
                roster: sorted_key_roster,
            };

            let json_string = serde_json::to_string_pretty(&cached_key).unwrap();
            let path = Path::new("cache.json");
            let mut cache_file = File::create(&path)?;
            cache_file.write_all(json_string.as_bytes())?;
        }
    }
    Ok(())
}
