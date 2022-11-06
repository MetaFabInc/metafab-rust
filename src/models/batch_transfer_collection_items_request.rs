/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.43
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BatchTransferCollectionItemsRequest {
    /// An array of valid EVM based addresses to transfer items to.
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// An array of wallet ids within the MetaFab ecosystem to transfer items to.
    #[serde(rename = "walletIds", skip_serializing_if = "Option::is_none")]
    pub wallet_ids: Option<Vec<String>>,
    /// An array of unique itemIds to transfer. Each recipient will receive the same set of items provided.
    #[serde(rename = "itemIds")]
    pub item_ids: Vec<f32>,
    /// The quantities of each unique itemId to transfer. Each recipient will receive the same quantities of items provided.
    #[serde(rename = "quantities")]
    pub quantities: Vec<f32>,
}

impl BatchTransferCollectionItemsRequest {
    pub fn new(item_ids: Vec<f32>, quantities: Vec<f32>) -> BatchTransferCollectionItemsRequest {
        BatchTransferCollectionItemsRequest {
            addresses: None,
            wallet_ids: None,
            item_ids,
            quantities,
        }
    }
}


