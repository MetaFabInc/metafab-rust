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
pub struct AuthGame200ResponseAllOf {
    #[serde(rename = "wallet", skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Box<crate::models::WalletModel>>,
}

impl AuthGame200ResponseAllOf {
    pub fn new() -> AuthGame200ResponseAllOf {
        AuthGame200ResponseAllOf {
            wallet: None,
        }
    }
}


