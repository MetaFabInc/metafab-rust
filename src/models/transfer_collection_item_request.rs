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
pub struct TransferCollectionItemRequest {
    /// A valid EVM based addresses to transfer items to.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// A wallet id within the MetaFab ecosystem to transfer items to.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<Vec<String>>,
    /// The quantity of the collectionItemId to transfer.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl TransferCollectionItemRequest {
    pub fn new(quantity: i32) -> TransferCollectionItemRequest {
        TransferCollectionItemRequest {
            address: None,
            wallet_id: None,
            quantity,
        }
    }
}


