use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, write};
use crate::util::user_path;

#[derive(Deserialize, Serialize)]
pub struct Cosmetics {
    pub status: i32,
    pub data: Vec<Item>
}

#[derive(Deserialize, Serialize)]
pub struct NewCosmetics {
    pub status: i32,
    pub data: NewCosmeticData
}

#[derive(Deserialize, Serialize)]
pub struct NewCosmeticData {
    pub items: Vec<Item>
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    id: String,
    #[serde(rename = "type")]
    item_type: Value
}

#[derive(Deserialize, Serialize)]
pub struct Value {
    value: String,
    #[serde(rename = "displayValue")]
    display_value: String,
    #[serde(rename = "backendValue")]
    backend_value: String
}

#[derive(Deserialize, Serialize)]
pub struct CItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub new: bool
}

// to-do add reading from a file in case fn-api is down
#[tokio::main]
pub async fn get() -> Result<Vec<CItem>, reqwest::Error> {
    let new = reqwest::get("https://fortnite-api.com/v2/cosmetics/br/new")
    .await?
    .json::<NewCosmetics>()
    .await?;
    
    let cosmetics = reqwest::get("https://fortnite-api.com/v2/cosmetics/br")
    .await?
    .json::<Cosmetics>()
    .await?;
    
    let cosmetic_items: Vec<CItem> = cosmetics.data.iter().map(|i| CItem {
        id: i.id.clone(),
        item_type: i.item_type.backend_value.clone(),
        new: match new.data.items.iter().find(|item| item.id == i.id) {
            Some(_) => true,
            None => false
        }
    }).collect();
    
    let path = user_path();
    create_dir_all(&path).unwrap();
    write(
        [&path, "cosmetics.json"].join("\\"),
        serde_json::to_string_pretty(&cosmetic_items).unwrap()
    ).unwrap();
    
    log::info!("Loaded {} items", &cosmetic_items.len());
    
    Ok(cosmetic_items)
}