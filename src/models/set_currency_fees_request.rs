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
pub struct SetCurrencyFeesRequest {
    /// The recipient address of currency transaction fees.
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    /// A percentage fee for every transaction represented in basis points. To set a 1.5% fee, you would use a value of 150. This value can be 0, denoting no percentage fees.
    #[serde(rename = "basisPoints")]
    pub basis_points: f32,
    /// A fixed fee for every transaction. A value of 0.5 would mean 0.5 of the currency of a transaction is always taken as a fee. This value can be 0, denoting no fixed fees.
    #[serde(rename = "fixedAmount")]
    pub fixed_amount: f32,
    /// The maximum fee amount for any single transaction. The total fee of a transaction is calculated as the sum of the basis points (percentage) fee, and fixed fee. If a calculated fee is greater than this maximum fee value, the maximum fee will be used instead.
    #[serde(rename = "capAmount")]
    pub cap_amount: f32,
}

impl SetCurrencyFeesRequest {
    pub fn new(recipient_address: String, basis_points: f32, fixed_amount: f32, cap_amount: f32) -> SetCurrencyFeesRequest {
        SetCurrencyFeesRequest {
            recipient_address,
            basis_points,
            fixed_amount,
            cap_amount,
        }
    }
}


