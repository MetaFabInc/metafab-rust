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
pub struct PublicPlayerCustodialWallet {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl PublicPlayerCustodialWallet {
    pub fn new() -> PublicPlayerCustodialWallet {
        PublicPlayerCustodialWallet {
            id: None,
            address: None,
        }
    }
}

