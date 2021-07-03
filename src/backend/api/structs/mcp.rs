use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::backend::cosmetics::CItem;
use crate::options::ALPHABET;

#[macro_export]
macro_rules! items {
    ($slot:expr) => {
        SlotData {
            items: $slot.items,
            activeVariants: $slot.variants,
        }
    };
}

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
    pub updated: String,
    pub favourites: Vec<String>,
    pub last_loadout: String,
    pub loadouts: HashMap<String, RLoadout>,
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
    pub banner_colour: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RSlot {
    pub items: Vec<String>,
    pub variants: Vec<Option<SlotVariants>>,
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
            updated: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            favourites: Vec::new(),
            last_loadout: random_loadout,
            loadouts,
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
            banner_colour: String::from("DefaultColor17"),
        }
    }
}

impl RSlot {
    pub fn new(len: u32) -> Self {
        let mut slot = Self {
            items: Vec::new(),
            variants: Vec::new(),
        };
        for _ in 0..len {
            slot.items.push(String::new());
            slot.variants.push(None);
        }
        slot
    }
}

// Fortnite Profile

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub profileRevision: i32,
    pub profileId: String,
    pub profileChangesBaseRevision: i32,
    pub profileChanges: Vec<ProfileChanges>,
    pub profileCommandRevision: i32,
    pub serverTime: String,
    pub responseVersion: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileChanges {
    Full(FullProfile),
    Changed(AttrChanged),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attributes {
    LockerSlots(LockerSlots),
    Bool(bool),
}

#[derive(Serialize, Deserialize)]
pub struct AttrChanged {
    pub changeType: String,
    pub itemId: String,
    pub attributeName: String,
    pub attributeValue: Attributes,
}

#[derive(Serialize, Deserialize)]
pub struct FullProfile {
    pub changeType: String,
    pub profile: FullProfileUpdate,
}

impl FullProfile {
    // TO-DO: Arc<Vec<_>> -> Vec<_> and get rid of some cloning
    pub fn new(cosmetics: Arc<Vec<CItem>>, profile: RProfile) -> Self {
        let id = profile.id;
        let mut full_profile = FullProfile {
            changeType: String::from("fullProfileUpdate"),
            profile: FullProfileUpdate {
                _id: id.clone(),
                created: profile.created,
                updated: profile.updated,
                rvn: 1,
                wipeNumber: 1,
                accountId: id,
                profileId: String::from("athena"),
                version: String::from("Ruten"),
                items: HashMap::new(),
                stats: Stats {
                    attributes: StatsAttributes {
                        use_random_loadout: false,
                        past_seasons: Vec::new(),
                        season_match_boost: 0,
                        loadouts: {
                            let keys = profile.loadouts.keys();
                            let mut data = Vec::new();
                            for i in keys {
                                data.push(i.clone());
                            }
                            data
                        },
                        mfa_reward_claimed: true,
                        rested_xp_overflow: 0,
                        quest_manager: json!({
                            "dailyLoginInterval": "2021-06-24T11:24:14.414Z",
                            "dailyQuestRerolls": 1
                        }),
                        book_level: 100,
                        season_num: 17,
                        season_update: 0,
                        book_xp: 999999,
                        permissions: Vec::new(),
                        season: json!({
                            "numWins": 0,
                            "numHighBracket": 0,
                            "numLowBracket": 0
                        }),
                        battlestars: 9999,
                        vote_data: json!({}),
                        battlestars_season_total: 9999,
                        alien_style_points: 9999,
                        book_purchased: true,
                        lifetime_wins: 999,
                        party_assist_quest: String::new(),
                        purchased_battle_pass_tier_offers: json!({}),
                        rested_xp_exchange: 1,
                        level: 100,
                        xp_overflow: 0,
                        rested_xp: 0,
                        rested_xp_mult: 4.55,
                        season_first_tracking_bits: Vec::new(),
                        accountLevel: 9999,
                        competitive_identity: json!({}),
                        inventory_limit_bonus: 0,
                        pinned_quest: String::new(),
                        last_applied_loadout: profile.last_loadout,
                        daily_rewards: json!({}),
                        xp: 9999999,
                        season_friend_match_boost: 0,
                        purchased_bp_offers: Vec::new(),
                        last_match_end_datetime: String::new(),
                        active_loadout_index: 0,
                    },
                },
                commandRevision: 1,
            },
        };

        for (id, loadout) in profile.loadouts {
            full_profile.profile.items.insert(
                id,
                Item::Loadout(LoadoutItem {
                    templateId: String::from("CosmeticLocker:cosmeticlocker_athena"),
                    attributes: LoadoutAttributes {
                        locker_slots_data: LockerSlots {
                            slots: Slots {
                                SkyDiveContrail: items!(loadout.contrail),
                                MusicPack: items!(loadout.music),
                                Character: items!(loadout.outfit),
                                Backpack: items!(loadout.backpack),
                                Glider: items!(loadout.glider),
                                Pickaxe: items!(loadout.pickaxe),
                                ItemWrap: items!(loadout.wraps),
                                LoadingScreen: items!(loadout.loading),
                                Dance: items!(loadout.dances),
                            },
                        },
                        use_count: 1,
                        banner_icon_template: loadout.banner_icon,
                        banner_color_template: loadout.banner_colour,
                        locker_name: loadout.name,
                        item_seen: false,
                        favorite: false,
                    },
                    quantity: 1,
                }),
            );
        }

        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        
        for item in cosmetics.iter() {
            let template_id = [item.item_type.clone(), item.id.clone()].join(":");
            full_profile.profile.items.insert(
                template_id.clone(),
                Item::Cosmetic(CosmeticItem {
                    templateId: template_id.clone(),
                    attributes: CosmeticAttributes {
                        creation_time: if item.new == true {
                            Some(now.clone())
                        } else {
                            None
                        },
                        max_level_bonus: 0,
                        level: 1,
                        item_seen: true,
                        rnd_sel_cnt: 0,
                        xp: 0,
                        variants: item.variants.iter().map(|v| Variant {
                            channel: v.channel.clone(),
                            active: v.options.get(0).unwrap().clone(),
                            owned: v.options.clone()
                        }).collect(),
                        favorite: profile.favourites.contains(&template_id),
                    },
                    quantity: 1,
                }),
            );
        }

        full_profile
    }
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
    pub commandRevision: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub attributes: StatsAttributes,
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
    pub active_loadout_index: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Cosmetic(CosmeticItem),
    Loadout(LoadoutItem),
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticItem {
    pub templateId: String,
    pub attributes: CosmeticAttributes,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    pub max_level_bonus: i32,
    pub level: i32,
    pub item_seen: bool,
    pub rnd_sel_cnt: i32,
    pub xp: i32,
    pub variants: Vec<Variant>,
    pub favorite: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub channel: String,
    pub active: String,
    pub owned: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoadoutItem {
    pub templateId: String,
    pub attributes: LoadoutAttributes,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LoadoutAttributes {
    pub locker_slots_data: LockerSlots,
    pub use_count: i32,
    pub banner_icon_template: String,
    pub banner_color_template: String,
    pub locker_name: String,
    pub item_seen: bool,
    pub favorite: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LockerSlots {
    pub slots: Slots,
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
    pub Dance: SlotData,
}

#[derive(Serialize, Deserialize)]
pub struct SlotData {
    pub items: Vec<String>,
    pub activeVariants: Vec<Option<SlotVariants>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SlotVariants {
    pub variants: Vec<SlotVariant>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SlotVariant {
    pub channel: String,
    pub active: String,
}
