/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPlayerConnectedWallet200Response {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Box<crate::models::TransactionModel>>,
}

impl SetPlayerConnectedWallet200Response {
    pub fn new() -> SetPlayerConnectedWallet200Response {
        SetPlayerConnectedWallet200Response {
            id: None,
            address: None,
            transaction: None,
        }
    }
}


