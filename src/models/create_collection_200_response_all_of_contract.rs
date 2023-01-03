/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCollection200ResponseAllOfContract {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "abi", skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,
    /// This field has not had a description added.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "forwarderAddress", skip_serializing_if = "Option::is_none")]
    pub forwarder_address: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::TransactionModel>>,
}

impl CreateCollection200ResponseAllOfContract {
    pub fn new() -> CreateCollection200ResponseAllOfContract {
        CreateCollection200ResponseAllOfContract {
            id: None,
            game_id: None,
            chain: None,
            abi: None,
            r#type: None,
            address: None,
            forwarder_address: None,
            updated_at: None,
            created_at: None,
            transactions: None,
        }
    }
}


