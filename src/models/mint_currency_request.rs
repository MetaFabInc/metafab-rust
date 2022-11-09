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
pub struct MintCurrencyRequest {
    /// The amount of currency to create (mint).
    #[serde(rename = "amount")]
    pub amount: f32,
    /// A valid EVM based address to create (mint) currency for. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Any wallet id within the MetaFab ecosystem to create (mint) currency for.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

impl MintCurrencyRequest {
    pub fn new(amount: f32) -> MintCurrencyRequest {
        MintCurrencyRequest {
            amount,
            address: None,
            wallet_id: None,
        }
    }
}


