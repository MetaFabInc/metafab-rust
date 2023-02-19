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
pub struct UpdateGame200ResponseAllOf {
    /// This field has not had a description added.
    #[serde(rename = "walletDecryptKey", skip_serializing_if = "Option::is_none")]
    pub wallet_decrypt_key: Option<String>,
}

impl UpdateGame200ResponseAllOf {
    pub fn new() -> UpdateGame200ResponseAllOf {
        UpdateGame200ResponseAllOf {
            wallet_decrypt_key: None,
        }
    }
}

