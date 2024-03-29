/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.4.4
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthGame200ResponseAllOf1 {
    #[serde(rename = "fundingWallet", skip_serializing_if = "Option::is_none")]
    pub funding_wallet: Option<Box<crate::models::WalletModel>>,
}

impl AuthGame200ResponseAllOf1 {
    pub fn new() -> AuthGame200ResponseAllOf1 {
        AuthGame200ResponseAllOf1 {
            funding_wallet: None,
        }
    }
}


