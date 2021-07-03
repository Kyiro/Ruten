use crate::backend::api::structs::mcp::*;
use crate::backend::cosmetics::CItem;
use crate::items;
use crate::util::user_path;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde::Deserialize;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

fn create(profile_id: String, change: Vec<ProfileChanges>, rvn: Option<i32>) -> Profile {
    Profile {
        profileRevision: rvn.unwrap_or(0) + 1,
        profileId: profile_id,
        profileChangesBaseRevision: rvn.unwrap_or(1),
        profileChanges: change,
        profileCommandRevision: rvn.unwrap_or(0) + 1,
        serverTime: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        responseVersion: 1,
    }
}

fn get_profile(id: &str) -> std::io::Result<RProfile> {
    let user = user_path();
    create_dir_all([&user, "profiles"].join("\\"))?;

    let path = [&user, "\\profiles\\", id, ".json"].join("");
    if !Path::new(&path).is_file() {
        let data = RProfile::new(id);
        write(&path, serde_json::to_string(&data)?)?;
        Ok(data)
    } else {
        Ok(serde_json::from_str(&read_to_string(&path)?)?)
    }
}

fn update_profile(profile: RProfile) -> std::io::Result<()> {
    let user = user_path();
    create_dir_all([&user, "profiles"].join("\\"))?;

    let path = [&user, "\\profiles\\", &profile.id, ".json"].join("");
    write(&path, serde_json::to_string(&profile)?)?;

    Ok(())
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Query {
    pub profileId: String,
    pub rvn: i32,
}

#[post("/fortnite/api/game/v2/profile/{id}/client/QueryProfile")]
pub async fn query_profile(
    cosmetics: web::Data<Vec<CItem>>,
    web::Path(id): web::Path<String>,
) -> std::io::Result<impl Responder> {
    let profile = get_profile(&id)?;

    let full_profile = FullProfile::new(cosmetics.into_inner(), profile);

    Ok(HttpResponse::Ok().json(create(
        String::from("athena"),
        vec![ProfileChanges::Full(full_profile)],
        None,
    )))
}

#[post("/fortnite/api/game/v2/profile/{id}/client/ClientQuestLogin")]
pub async fn client_quest_login() -> impl Responder {
    HttpResponse::Ok().json(create(String::from("athena"), Vec::new(), None))
}

#[derive(Deserialize)]
pub enum Category {
    Character,
    Dance,
    Glider,
    Pickaxe,
    Backpack,
    LoadingScreen,
    SkyDiveContrail,
    MusicPack,
    ItemWrap,
}

#[derive(Deserialize)]
pub struct SetCosmeticLockerSlot {
    pub lockerItem: String,
    pub category: Category,
    pub itemToSlot: String,
    pub slotIndex: i32,
    pub variantUpdates: Vec<SlotVariant>,
}

#[post("/fortnite/api/game/v2/profile/{id}/client/SetCosmeticLockerSlot")]
pub async fn set_cosmetic_locker_slot(
    web::Path(id): web::Path<String>,
    data: web::Json<SetCosmeticLockerSlot>,
    query: web::Query<Query>,
) -> std::io::Result<impl Responder> {
    let data = data.into_inner();
    let mut profile = get_profile(&id)?;
    let loadout = profile.loadouts.get_mut(&data.lockerItem).unwrap();

    let to_slot = match data.category {
        Category::Character => &mut loadout.outfit,
        Category::Dance => &mut loadout.dances,
        Category::Glider => &mut loadout.glider,
        Category::Pickaxe => &mut loadout.pickaxe,
        Category::Backpack => &mut loadout.backpack,
        Category::LoadingScreen => &mut loadout.loading,
        Category::SkyDiveContrail => &mut loadout.contrail,
        Category::MusicPack => &mut loadout.music,
        Category::ItemWrap => &mut loadout.wraps,
    };

    let slot = to_slot.items.get_mut(data.slotIndex as usize).unwrap();
    *slot = data.itemToSlot;
    
    let v_slot = to_slot.variants.get_mut(data.slotIndex as usize).unwrap();
    if data.variantUpdates.len() == 0 {
        *v_slot = None;
    } else {
        *v_slot = Some(SlotVariants {
            variants: data.variantUpdates
        });
    }

    update_profile(profile)?;

    // my lazy ass
    let profile = get_profile(&id)?;
    let loadout = profile.loadouts.get(&data.lockerItem).unwrap();

    let attr_changed = AttrChanged {
        changeType: String::from("itemAttrChanged"),
        itemId: data.lockerItem,
        attributeName: String::from("locker_slots_data"),
        attributeValue: Attributes::LockerSlots(LockerSlots {
            slots: Slots {
                SkyDiveContrail: items!(loadout.contrail.clone()),
                MusicPack: items!(loadout.music.clone()),
                Character: items!(loadout.outfit.clone()),
                Backpack: items!(loadout.backpack.clone()),
                Glider: items!(loadout.glider.clone()),
                Pickaxe: items!(loadout.pickaxe.clone()),
                ItemWrap: items!(loadout.wraps.clone()),
                LoadingScreen: items!(loadout.loading.clone()),
                Dance: items!(loadout.dances.clone()),
            },
        }),
    };

    Ok(HttpResponse::Ok().json(create(
        String::from("athena"),
        vec![ProfileChanges::Changed(attr_changed)],
        Some(query.rvn),
    )))
}

#[derive(Deserialize)]
pub struct SetItemFavoriteStatusBatch {
    pub itemFavStatus: Vec<bool>,
    pub itemIds: Vec<String>,
}

#[post("/fortnite/api/game/v2/profile/{id}/client/SetItemFavoriteStatusBatch")]
pub async fn set_item_favorite_status_batch(
    web::Path(id): web::Path<String>,
    data: web::Json<SetItemFavoriteStatusBatch>,
    query: web::Query<Query>,
) -> std::io::Result<impl Responder> {
    let data = data.into_inner();
    let mut profile = get_profile(&id)?;

    if data.itemFavStatus.len() != data.itemIds.len() {
        return Ok(HttpResponse::BadRequest().into());
    }

    let mut changes: Vec<AttrChanged> = Vec::new();

    for idx in 0..data.itemFavStatus.len() {
        let (status, id) = (
            *data.itemFavStatus.get(idx).unwrap(),
            data.itemIds.get(idx).unwrap().clone(),
        );

        if status == true {
            profile.favourites.push(id.clone());
        } else {
            profile.favourites = profile
                .favourites
                .into_iter()
                .filter(|i| **i != id)
                .collect()
        }

        changes.push(AttrChanged {
            changeType: String::from("itemAttrChanged"),
            itemId: id,
            attributeName: String::from("favorite"),
            attributeValue: Attributes::Bool(status),
        });
    }

    update_profile(profile)?;

    Ok(HttpResponse::Ok().json(create(
        String::from("athena"),
        changes
            .into_iter()
            .map(|i| ProfileChanges::Changed(i))
            .collect(),
        Some(query.rvn),
    )))
}

#[post("/fortnite/api/game/v2/profile/{id}/client/{action}")]
pub async fn other(
    web::Path((_, _)): web::Path<((), ())>,
    query: web::Query<Query>,
) -> impl Responder {
    let query = query.into_inner();
    HttpResponse::Ok().json(create(query.profileId, Vec::new(), Some(query.rvn)))
}
