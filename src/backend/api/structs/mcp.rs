use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::options::ALPHABET;

/*
    when adding a new value
    add #[serde(default = "foo")]
    which will call foo() to get the default value
    if it's not there while deserializing
    fn foo() -> String { String::from("foo") }
    (sorry if i'm fooing in a wrong way)
    - Kyiro
*/

// Ruten Profile

#[derive(Serialize, Deserialize)]
pub struct RProfile {
    pub id: String,
    pub created: String,
    pub favourites: Vec<String>,
    pub last_loadout: String,
    pub loadouts: HashMap<String, RLoadout>,
    pub rvn: i32
}

#[derive(Serialize, Deserialize)]
pub struct RLoadout {
    pub outfit: RSlot,
    pub backpack: RSlot,
    pub pickaxe: RSlot,
    pub glider: RSlot,
    pub contrail: RSlot,
    pub dances: RSlot,
    pub wraps: RSlot,
    pub music: RSlot,
    pub loading: RSlot,
    
    pub name: String,
    pub banner_icon: String,
    pub banner_colour: String
}

#[derive(Serialize, Deserialize)]
pub struct RSlot {
    pub items: Vec<String>,
    pub variants: Vec<Option<SlotVariants>>
}

impl RProfile {
    pub fn new(id: &str) -> Self {
        let random_loadout = nanoid::nanoid!(32, &ALPHABET);
        let loadouts = {
            let mut map = HashMap::new();
            map.insert(random_loadout.clone(), RLoadout::new("Ruten"));
            map
        };
        
        Self {
            id: id.to_string(),
            created: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            favourites: Vec::new(),
            last_loadout: random_loadout,
            loadouts,
            rvn: 1
        }
    }
}

impl RLoadout {
    pub fn new(name: &str) -> Self {
        Self {
            outfit: RSlot::new(1),
            backpack: RSlot::new(1),
            pickaxe: RSlot::new(1),
            glider: RSlot::new(1),
            contrail: RSlot::new(1),
            dances: RSlot::new(6),
            wraps: RSlot::new(7),
            music: RSlot::new(1),
            loading: RSlot::new(1),
            
            name: name.to_string(),
            banner_icon: String::from("OtherBanner51"),
            banner_colour: String::from("DefaultColor17")
        }
    }
}

impl RSlot {
    pub fn new(len: u32) -> Self {
        let mut slot = Self {
            items: Vec::new(),
            variants: Vec::new()
        };
        for _ in 1..len {
            slot.items.push(String::new());
            slot.variants.push(None);
        }
        slot
    }
}

// Fortnite Profile

// did camelCase to snake_case for no fucking reason lol
// send help
#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub profileRevision: i32,
    pub profileId: String,
    pub profileChangesBaseRevision: i32,
    pub profileChanges: Vec<ProfileChanges>,
    pub profileCommandRevision: i32,
    pub serverTime: String,
    pub responseVersion: i32
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileChanges {
    Full(FullProfile)
}

#[derive(Serialize, Deserialize)]
pub struct FullProfile {
    pub changeType: String,
    pub profile: FullProfileUpdate
}

#[derive(Serialize, Deserialize)]
pub struct FullProfileUpdate {
    pub _id: String,
    pub created: String,
    pub updated: String,
    pub rvn: i32,
    pub wipeNumber: i32,
    pub accountId: String,
    pub profileId: String,
    pub version: String,
    pub items: HashMap<String, Item>,
    pub stats: Stats,
    pub commandRevision: i32
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub attributes: StatsAttributes
}

#[derive(Serialize, Deserialize)]
pub struct StatsAttributes {
    pub use_random_loadout: bool,
    pub past_seasons: Vec<Value>,
    pub season_match_boost: i32,
    pub loadouts: Vec<String>,
    pub mfa_reward_claimed: bool,
    pub rested_xp_overflow: i32,
    pub quest_manager: Value,
    pub book_level: i32,
    pub season_num: i32,
    pub season_update: i32,
    pub book_xp: i32,
    pub permissions: Vec<Value>,
    pub season: Value,
    pub battlestars: i32,
    pub vote_data: Value,
    pub battlestars_season_total: i32,
    pub alien_style_points: i32,
    pub book_purchased: bool,
    pub lifetime_wins: i32,
    pub party_assist_quest: String,
    pub purchased_battle_pass_tier_offers: Value,
    pub rested_xp_exchange: i32,
    pub level: i32,
    pub xp_overflow: i32,
    pub rested_xp: i32,
    pub rested_xp_mult: f32,
    pub season_first_tracking_bits: Vec<Value>,
    pub accountLevel: i32,
    pub competitive_identity: Value,
    pub inventory_limit_bonus: i32,
    pub pinned_quest: String,
    pub last_applied_loadout: String,
    pub daily_rewards: Value,
    pub xp: i32,
    pub season_friend_match_boost: i32,
    pub purchased_bp_offers: Vec<Value>,
    pub last_match_end_datetime: String,
    pub active_loadout_index: i32
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Cosmetic(CosmeticItem),
    Loadout(LoadoutItem)
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticItem {
    pub templateId: String,
    pub attributes: CosmeticAttributes,
    pub quantity: i32
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticAttributes {
    pub creation_time: String,
    pub max_level_bonus: i32,
    pub level: i32,
    pub item_seen: bool,
    pub rnd_sel_cnt: i32,
    pub xp: i32,
    pub variants: Vec<Variant>,
    pub favorite: bool
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub channel: String,
    pub active: String,
    pub owned: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct LoadoutItem {
    pub templateId: String,
    pub attributes: LoadoutAttributes,
    pub quantity: i32
}

#[derive(Serialize, Deserialize)]
pub struct LoadoutAttributes {
    pub locker_slots_data: LockerSlots,
    pub use_count: i32,
    pub banner_icon_template: String,
    pub banner_color_template: String,
    pub locker_name: String,
    pub item_seen: bool,
    pub favorite: bool
}

#[derive(Serialize, Deserialize)]
pub struct LockerSlots {
    pub slots: Slots
}

#[derive(Serialize, Deserialize)]
pub struct Slots {
    pub SkyDiveContrail: SlotData,
    pub MusicPack: SlotData,
    pub Character: SlotData,
    pub Backpack: SlotData,
    pub Glider: SlotData,
    pub Pickaxe: SlotData,
    pub ItemWrap: SlotData,
    pub LoadingScreen: SlotData,
    pub Dance: SlotData
}

#[derive(Serialize, Deserialize)]
pub struct SlotData {
    pub items: Vec<String>,
    pub activeVariants: Vec<Option<SlotVariants>>
}

#[derive(Serialize, Deserialize)]
pub struct SlotVariants {
    pub variants: Vec<SlotVariant>
}

#[derive(Serialize, Deserialize)]
pub struct SlotVariant {
    pub channel: String,
    pub active: String
}