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
pub struct SetShopOfferRequest {
    /// A unique offer id to use for this offer for the shop. If an existing offer id is used, the current offer will be updated but the existing number of uses will be kept. If you want to reset the number of uses for an existing offer, first remove it using the remove offer endpoint, then set it.
    #[serde(rename = "id")]
    pub id: i32,
    /// A valid EVM based ERC1155 or MetaFab game items contract address that represents the collection for input items required by this offer. `inputCollectionAddress` or `inputCollectionId` can optionally be provided.
    #[serde(rename = "inputCollectionAddress", skip_serializing_if = "Option::is_none")]
    pub input_collection_address: Option<String>,
    /// A valid MetaFab collection id that represents the collection for input items required by this offer. `inputCollectionAddress` or `inputCollectionId` can optionally be provided.
    #[serde(rename = "inputCollectionId", skip_serializing_if = "Option::is_none")]
    pub input_collection_id: Option<String>,
    /// An array of item ids from the provided input collection that are required to use this offer. Input items are transferred from the wallet to the shop contract upon using an offer.
    #[serde(rename = "inputCollectionItemIds", skip_serializing_if = "Option::is_none")]
    pub input_collection_item_ids: Option<Vec<i32>>,
    /// An array of amounts for each item id from the provided input collection that are required to use this offer. Item amounts array indices are reflective of the amount required for a given item id at the same index.
    #[serde(rename = "inputCollectionItemAmounts", skip_serializing_if = "Option::is_none")]
    pub input_collection_item_amounts: Option<Vec<i32>>,
    /// A valid EVM based ERC20 or MetaFab game currency contract address that for the currency required by this offer.
    #[serde(rename = "inputCurrencyAddress", skip_serializing_if = "Option::is_none")]
    pub input_currency_address: Option<String>,
    /// A valid MetaFab currency id that represents the currency required by this offer.
    #[serde(rename = "inputCurrencyId", skip_serializing_if = "Option::is_none")]
    pub input_currency_id: Option<String>,
    /// The amount of currency required by this offer. If an inputCurrencyAmount is provided without in input currency address or id, the native chain currency is used as the required currency.
    #[serde(rename = "inputCurrencyAmount", skip_serializing_if = "Option::is_none")]
    pub input_currency_amount: Option<f32>,
    /// A valid EVM based ERC1155 or MetaFab game items contract address that represents the collection for output items given by this offer. `outputCollectionAddress` or `outputCollectionId` can optionally be provided.
    #[serde(rename = "outputCollectionAddress", skip_serializing_if = "Option::is_none")]
    pub output_collection_address: Option<String>,
    /// A valid MetaFab collection id that represents the collection for output items given by this offer. `outputCollectionAddress` or `outputCollectionId` can optionally be provided.
    #[serde(rename = "outputCollectionId", skip_serializing_if = "Option::is_none")]
    pub output_collection_id: Option<String>,
    /// An array of item ids from the provided output collection that are given by this offer. Output items are automatically minted if the shop contract has the `minter` role for the output collection contract. Otherwise, they are transferred from the item balance held by the shop contract.
    #[serde(rename = "outputCollectionItemIds", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_ids: Option<Vec<i32>>,
    /// An array of amounts for each item id from the provided output collection that are given by this offer. Item amounts array indices are reflective of the amount required for a given item id at the same index.
    #[serde(rename = "outputCollectionItemAmounts", skip_serializing_if = "Option::is_none")]
    pub output_collection_item_amounts: Option<Vec<i32>>,
    /// A valid EVM based ERC20 or MetaFab game currency contract address that for the currency given by this offer. The output currency amount is automatically minted if the shop contract has the `minter` role for the output currency contract. Otherwise, they are transferred from the currency balance held by the shop contract.
    #[serde(rename = "outputCurrencyAddress", skip_serializing_if = "Option::is_none")]
    pub output_currency_address: Option<String>,
    /// A valid MetaFab currency id for the currency given by this offer.
    #[serde(rename = "outputCurrencyId", skip_serializing_if = "Option::is_none")]
    pub output_currency_id: Option<String>,
    /// The amount of currency given by this offer. If an outputCurrencyAmount is provided without an output currency address or id, the native chain currency is used as the given currency.
    #[serde(rename = "outputCurrencyAmount", skip_serializing_if = "Option::is_none")]
    pub output_currency_amount: Option<f32>,
    /// The maximum number of times this offer can be used in total. maxUses is collective across all uses of the offer. If 5 unique players use an offer, that counts as 5 offer uses. Exclude this or use 0 to allow unlimited uses.
    #[serde(rename = "maxUses", skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<i32>,
}

impl SetShopOfferRequest {
    pub fn new(id: i32) -> SetShopOfferRequest {
        SetShopOfferRequest {
            id,
            input_collection_address: None,
            input_collection_id: None,
            input_collection_item_ids: None,
            input_collection_item_amounts: None,
            input_currency_address: None,
            input_currency_id: None,
            input_currency_amount: None,
            output_collection_address: None,
            output_collection_id: None,
            output_collection_item_ids: None,
            output_collection_item_amounts: None,
            output_currency_address: None,
            output_currency_id: None,
            output_currency_amount: None,
            max_uses: None,
        }
    }
}


