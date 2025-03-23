use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]

pub struct KeyResponse {
    pub season: String,
    pub status: String,
    pub dungeon: Dungeon,
    pub keystone_run_id: i64,
    pub mythic_level: i64,
    pub num_chests: i64,
    pub roster: Vec<Roster>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dungeon {
    pub name: String,
    pub short_name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Roster {
    pub character: Character,
    pub items: Items,
    pub role: String,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Items {
    pub item_level_equipped: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
}
