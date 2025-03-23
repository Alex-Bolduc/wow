use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub name: String,
    pub race: String,
    pub class: String,
    pub active_spec_name: String,
    pub active_spec_role: String,
    pub gender: String,
    pub faction: String,
    pub achievement_points: i64,
    pub thumbnail_url: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: String,
    pub profile_url: String,
    pub profile_banner: String,
    pub mythic_plus_recent_runs: Vec<MythicPlusRecentRun>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MythicPlusRecentRun {
    pub dungeon: String,
    pub short_name: String,
    pub mythic_level: i64,
    pub completed_at: String,
    pub clear_time_ms: i64,
    pub keystone_run_id: i64,
    pub par_time_ms: i64,
    pub num_keystone_upgrades: i64,
    pub map_challenge_mode_id: i64,
    pub zone_id: i64,
    pub zone_expansion_id: i64,
    pub icon_url: String,
    pub background_image_url: String,
    pub score: f64,
    pub affixes: Vec<Affix>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Affix {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub icon_url: String,
    pub wowhead_url: String,
}
