/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.2.1
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExchangeOffer {
    /// The id of this offer.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    /// The address of the ERC1155 of MetaFab game items contract for input items required by this offer.
    #[serde(rename = "inputCollection", skip_serializing_if = "Option::is_none")]
    pub input_collection: Option<String>,
    /// An array of item ids from the input collection that are required for this offer.
    #[serde(rename = "inputCollectionItemIds", skip_serializing_if = "Option::is_none")]
    pub input_collection_item_ids: Option<Vec<f32>>,
    /// An array of amounts for each item id for the input collection that are required to use this offer.
    #[serde(rename = "inputCollectionItemAmounts", skip_serializing_if = "Option::is_none")]
    pub input_collection_item_amounts: Option<Vec<f32>>,
    /// The address of the ERC20 or MetaFab game currency for the currency required by this offer.
    #[serde(rename = "inputCurrency", skip_serializing_if = "Option::is_none")]
    pub input_currency: Option<String>,
    /// The amount of currency required by this offer.
    #[serde(rename = "inputCurrencyAmount", skip_serializing_if = "Option::is_none")]
    pub input_currency_amount: Option<f32>,
    /// The address of the ERC1155 of MetaFab game items contract for output items given by this offer.
    #[serde(rename = "outputCollection", skip_serializing_if = "Option::is_none")]
    pub output_collection: Option<String>,
    /// An array of item ids from the output collection that are given for this offer.
    #[serde(rename = "outputCollectionItemIds", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_ids: Option<Vec<f32>>,
    /// An array of amounts for each item id for the output collection that are given by this offer.
    #[serde(rename = "outputCollectionItemAmounts", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_amounts: Option<Vec<f32>>,
    /// The address of the ERC20 or MetaFab game currency for the output currency given by this offer.
    #[serde(rename = "outputCurrency", skip_serializing_if = "Option::is_none")]
    pub output_currency: Option<String>,
    /// The amount of currency given by this offer.
    #[serde(rename = "outputCurrencyAmount", skip_serializing_if = "Option::is_none")]
    pub output_currency_amount: Option<f32>,
    /// The number of times this offer has been used.
    #[serde(rename = "uses", skip_serializing_if = "Option::is_none")]
    pub uses: Option<f32>,
    /// The maximum number of times this offer can be used. A value of `0` means there is no limit on how many times this offer can be used.
    #[serde(rename = "maxUses", skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<f32>,
    /// A unix timestamp in seconds that represents the last time this offer was set or updated.
    #[serde(rename = "lastUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f32>,
}

impl ExchangeOffer {
    pub fn new() -> ExchangeOffer {
        ExchangeOffer {
            id: None,
            input_collection: None,
            input_collection_item_ids: None,
            input_collection_item_amounts: None,
            input_currency: None,
            input_currency_amount: None,
            output_collection: None,
            output_collection_item_ids: None,
            output_collection_item_amounts: None,
            output_currency: None,
            output_currency_amount: None,
            uses: None,
            max_uses: None,
            last_updated_at: None,
        }
    }
}


