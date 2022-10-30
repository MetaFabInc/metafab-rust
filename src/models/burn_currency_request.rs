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
pub struct BurnCurrencyRequest {
    /// The amount of currency to remove (burn). The currency balance of the authenticating game or player's wallet must be equal to or greater than this amount.
    #[serde(rename = "amount")]
    pub amount: f32,
}

impl BurnCurrencyRequest {
    pub fn new(amount: f32) -> BurnCurrencyRequest {
        BurnCurrencyRequest {
            amount,
        }
    }
}


