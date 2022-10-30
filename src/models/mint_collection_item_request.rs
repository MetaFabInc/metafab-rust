/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MintCollectionItemRequest {
    /// A valid EVM based address to create (mint) the item(s) for. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The quantity of the specified item id to create (mint).
    #[serde(rename = "quantity")]
    pub quantity: f32,
    /// Any wallet id within the MetaFab ecosystem to create (mint) the item(s) for.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

impl MintCollectionItemRequest {
    pub fn new(quantity: f32) -> MintCollectionItemRequest {
        MintCollectionItemRequest {
            address: None,
            quantity,
            wallet_id: None,
        }
    }
}


