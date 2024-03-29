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
pub struct TransactionModel {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
    /// This field has not had a description added.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl TransactionModel {
    pub fn new() -> TransactionModel {
        TransactionModel {
            id: None,
            contract_id: None,
            wallet_id: None,
            function: None,
            args: None,
            hash: None,
            updated_at: None,
            created_at: None,
        }
    }
}


