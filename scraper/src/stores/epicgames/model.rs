use serde::{Deserialize, Deserializer};
use utils::model::GameType;

#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse {
    // pub errors: Value,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Data {
    #[serde(rename = "Catalog")]
    pub catalog: Catalog,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Catalog {
    #[serde(rename = "searchStore")]
    pub search_store: SearchStore,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SearchStore {
    pub elements: Vec<EpicGamesGame>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EpicGamesGame {
    pub title: String,
    pub id: String,
    // pub namespace: String,
    // pub description: String,
    // pub effective_date: String,
    #[serde(deserialize_with = "deserialize_game_type")]
    pub offer_type: GameType,
    // pub expiry_date: Option<String>,
    // pub viewable_date: String,
    pub status: String,
    // pub is_code_redemption_only: bool,
    // pub key_images: Vec<KeyImage>,
    // pub seller: Seller,
    // pub product_slug: Option<String>,
    // pub url_slug: Option<String>,
    // pub url: Option<String>,
    // pub items: Vec<Item>,
    pub custom_attributes: Vec<Attribute>,
    pub catalog_ns: CatalogNs,
    // pub categories: Vec<Category>,
    // pub offer_mappings: Option<Vec<Mapping>>,
    pub price: Price,
    pub promotions: Option<Promotions>,
}

fn deserialize_game_type<'de, D>(deserializer: D) -> Result<GameType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "BASE_GAME" | "OTHERS" => Ok(GameType::Game),
        "ADD_ON" => Ok(GameType::Dlc),
        "BUNDLE" => Ok(GameType::Bundle),
        "EDITION" => Ok(GameType::Edition),
        _ => Ok(GameType::Unknown),
    }
}

// #[derive(Debug, Deserialize)]
// pub(crate) struct KeyImage {
//     #[serde(rename = "type")]
//     pub image_type: String,
//     pub url: String,
// }

// #[derive(Debug, Deserialize)]
// pub(crate) struct Seller {
//     pub id: String,
//     pub name: String,
// }

// #[derive(Debug, Deserialize)]
// pub(crate) struct Item {
//     pub id: String,
//     pub namespace: String,
// }

#[derive(Debug, Deserialize)]
pub(crate) struct Attribute {
    pub key: String,
    pub value: String,
}

// #[derive(Debug, Deserialize)]
// pub(crate) struct Category {
//     pub path: String,
// }

// #[derive(Debug, Deserialize)]
// pub(crate) struct Tag {
//     pub id: String,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Mapping {
    pub page_slug: String,
    pub page_type: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CatalogNs {
    pub mappings: Option<Vec<Mapping>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Price {
    pub total_price: TotalPrice,
    // pub line_offers: Vec<LineOffer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TotalPrice {
    pub discount_price: u16,
    pub original_price: u16,
    // pub voucher_discount: u16,
    // pub discount: u16,
    // pub currency_code: String,
    // pub currency_info: HashMap<String, u8>,
    pub fmt_price: FmtPrice,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FmtPrice {
    pub original_price: String,
    // pub discount_price: String,
    // pub intermediate_price: String,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct LineOffer {
//     pub applied_rules: Vec<AppliedRule>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct AppliedRule {
//     pub id: String,
//     pub end_date: String,
//     pub discount_setting: HashMap<String, String>,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Promotions {
    pub promotional_offers: Vec<PromotionalOfferWrapper>,
    // pub upcoming_promotional_offers: Option<Vec<PromotionalOfferWrapper>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PromotionalOfferWrapper {
    pub promotional_offers: Vec<PromotionalOffer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PromotionalOffer {
    // pub start_date: String,
    pub end_date: String,
    pub discount_setting: DiscountSetting,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiscountSetting {
    // pub discount_type: String,
    pub discount_percentage: u16,
}
