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
pub struct GetCurrencies200ResponseInner {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "contractId", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "supplyCap", skip_serializing_if = "Option::is_none")]
    pub supply_cap: Option<i32>,
    /// This field has not had a description added.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "contract", skip_serializing_if = "Option::is_none")]
    pub contract: Option<Box<crate::models::ContractModel>>,
}

impl GetCurrencies200ResponseInner {
    pub fn new() -> GetCurrencies200ResponseInner {
        GetCurrencies200ResponseInner {
            id: None,
            game_id: None,
            contract_id: None,
            name: None,
            symbol: None,
            supply_cap: None,
            updated_at: None,
            created_at: None,
            contract: None,
        }
    }
}


