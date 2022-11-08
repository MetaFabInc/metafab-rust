/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.2.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WithdrawFromExchangeRequest {
    /// A valid EVM based address to withdraw to. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Any wallet id within the MetaFab ecosystem to withdraw to.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// The address of the currency (ERC20) token to withdraw from the exchange. If no currencyAddress or currencyId, and no collectionAddress or collectionId are provided, the native token held by the exchange will be withdrawn.
    #[serde(rename = "currencyAddress", skip_serializing_if = "Option::is_none")]
    pub currency_address: Option<String>,
    /// A valid MetaFab currency id that represents the currency token to withdraw from the exchange. `currencyAddress` or `currencyId` can be provided when withdrawing currency.
    #[serde(rename = "currencyId", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// The address of the collection (ERC1155) for the items to withdraw from the exchange. If no currencyAddress and no collectionAddress is provided, the native token held by the exchange will be withdrawn.
    #[serde(rename = "collectionAddress", skip_serializing_if = "Option::is_none")]
    pub collection_address: Option<String>,
    /// A valid MetaFab collection id that represents the collection for the items to withdraw from the exchange. `collectionAddress` or `collectionId` can be provided when withdrawing items.
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// The specific itemIds of the provided collection to withdraw from the exchange.
    #[serde(rename = "itemIds", skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<f32>>,
}

impl WithdrawFromExchangeRequest {
    pub fn new() -> WithdrawFromExchangeRequest {
        WithdrawFromExchangeRequest {
            address: None,
            wallet_id: None,
            currency_address: None,
            currency_id: None,
            collection_address: None,
            collection_id: None,
            item_ids: None,
        }
    }
}


