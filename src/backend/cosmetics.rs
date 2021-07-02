use crate::util::user_path;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, write};

#[derive(Deserialize, Serialize)]
pub struct Cosmetics {
    pub status: i32,
    pub data: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct NewCosmetics {
    pub status: i32,
    pub data: NewCosmeticData,
}

#[derive(Deserialize, Serialize)]
pub struct NewCosmeticData {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    id: String,
    #[serde(rename = "type")]
    item_type: Value,
    variants: Option<Vec<Variant>>,
}

#[derive(Deserialize, Serialize)]
pub struct Variant {
    channel: String,
    options: Vec<VariantOption>,
}

#[derive(Deserialize, Serialize)]
pub struct VariantOption {
    tag: String,
}

#[derive(Deserialize, Serialize)]
pub struct Value {
    value: String,
    #[serde(rename = "displayValue")]
    display_value: String,
    #[serde(rename = "backendValue")]
    backend_value: String,
}

#[derive(Deserialize, Serialize)]
pub struct CItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub new: bool,
    pub variants: Vec<CVariant>,
}

#[derive(Deserialize, Serialize)]
pub struct CVariant {
    pub channel: String,
    pub options: Vec<String>,
}

// to-do add reading from a file in case fn-api is down
#[tokio::main]
pub async fn get() -> Result<Vec<CItem>, reqwest::Error> {
    let new = reqwest::get("https://fortnite-api.com/v2/cosmetics/br/new")
        .await?
        .json::<NewCosmetics>()
        .await?;

    let new: Vec<String> = new.data.items.into_iter().map(|i| i.id).collect();

    let cosmetics = reqwest::get("https://fortnite-api.com/v2/cosmetics/br")
        .await?
        .json::<Cosmetics>()
        .await?;

    let cosmetic_items: Vec<CItem> = cosmetics
        .data
        .into_iter()
        .map(|i| CItem {
            id: i.id.clone(),
            item_type: i.item_type.backend_value,
            new: new.contains(&i.id),
            variants: if let Some(variants) = i.variants {
                variants
                    .into_iter()
                    .map(|v| CVariant {
                        channel: v.channel,
                        options: v.options.into_iter().map(|o| o.tag).collect(),
                    })
                    .collect()
            } else {
                Vec::new()
            },
        })
        .collect();

    let path = user_path();
    create_dir_all(&path).unwrap();
    write(
        [&path, "cosmetics.json"].join("\\"),
        serde_json::to_string_pretty(&cosmetic_items).unwrap(),
    )
    .unwrap();

    log::info!("Loaded {} items", &cosmetic_items.len());

    Ok(cosmetic_items)
}
