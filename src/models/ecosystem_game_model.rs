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
pub struct EcosystemGameModel {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "ecosystemId", skip_serializing_if = "Option::is_none")]
    pub ecosystem_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl EcosystemGameModel {
    pub fn new() -> EcosystemGameModel {
        EcosystemGameModel {
            id: None,
            ecosystem_id: None,
            game_id: None,
            updated_at: None,
            created_at: None,
        }
    }
}


