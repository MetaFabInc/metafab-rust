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
pub struct GetCurrencyFees200Response {
    /// The wallet address that fees from all applicable transactions are automatically sent to.
    #[serde(rename = "recipientAddress", skip_serializing_if = "Option::is_none")]
    pub recipient_address: Option<String>,
    /// The number of fee basis points. 100 basisPoints = 1% fee of the total transaction amount deducted from the total received by the recipient.
    #[serde(rename = "basisPoints", skip_serializing_if = "Option::is_none")]
    pub basis_points: Option<f32>,
    /// The fixed number of currency as a fee regardless of the total transaction amount. 10 = 10 of the currency as a fee for any transaction, deducted from the total received by the recipient.
    #[serde(rename = "fixedAmount", skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<f32>,
    /// The maximum combined fee between basisPoints and fixedAmount. If the total transaction fee is over this amount, the capAmount will be used as the transaction fee deducted from the total received by the recipient.
    #[serde(rename = "capAmount", skip_serializing_if = "Option::is_none")]
    pub cap_amount: Option<f32>,
}

impl GetCurrencyFees200Response {
    pub fn new() -> GetCurrencyFees200Response {
        GetCurrencyFees200Response {
            recipient_address: None,
            basis_points: None,
            fixed_amount: None,
            cap_amount: None,
        }
    }
}


