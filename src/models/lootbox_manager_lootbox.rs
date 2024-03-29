/*
 * MetaFab API
 *
 * Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.5.1
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LootboxManagerLootbox {
    /// The id of this lootbox.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The address of the ERC1155 or MetaFab game items contract for input items required by this lootbox.
    #[serde(rename = "inputCollection", skip_serializing_if = "Option::is_none")]
    pub input_collection: Option<String>,
    /// An array of item ids from the input collection that are required for this lootbox.
    #[serde(rename = "inputCollectionItemIds", skip_serializing_if = "Option::is_none")]
    pub input_collection_item_ids: Option<Vec<i32>>,
    /// An array of amounts for each item id for the input collection that are required to open this lootbox.
    #[serde(rename = "inputCollectionItemAmounts", skip_serializing_if = "Option::is_none")]
    pub input_collection_item_amounts: Option<Vec<i32>>,
    /// The address of the ERC1155 of MetaFab game items contract for possible output items given by this lootbox.
    #[serde(rename = "outputCollection", skip_serializing_if = "Option::is_none")]
    pub output_collection: Option<String>,
    /// An array of item ids from the output collection that are possibly given by this lootbox.
    #[serde(rename = "outputCollectionItemIds", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_ids: Option<Vec<i32>>,
    /// An array of amounts for each item id for the output collection that are possibly given by this lootbox.
    #[serde(rename = "outputCollectionItemAmounts", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_amounts: Option<Vec<i32>>,
    /// An array of weights for each item id for the output collection that are possibly given by this lootbox.
    #[serde(rename = "outputCollectionItemWeights", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_weights: Option<Vec<i32>>,
    /// The number of items randomly selected when this lootbox is opened.
    #[serde(rename = "outputTotalItems", skip_serializing_if = "Option::is_none")]
    pub output_total_items: Option<i32>,
    /// A unix timestamp in seconds that represents the last time this offer was set or updated.
    #[serde(rename = "lastUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<i32>,
}

impl LootboxManagerLootbox {
    pub fn new() -> LootboxManagerLootbox {
        LootboxManagerLootbox {
            id: None,
            input_collection: None,
            input_collection_item_ids: None,
            input_collection_item_amounts: None,
            output_collection: None,
            output_collection_item_ids: None,
            output_collection_item_amounts: None,
            output_collection_item_weights: None,
            output_total_items: None,
            last_updated_at: None,
        }
    }
}


