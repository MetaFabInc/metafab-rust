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
pub struct GetProfileGames200ResponseInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publishedKey", skip_serializing_if = "Option::is_none")]
    pub published_key: Option<String>,
    #[serde(rename = "iconImageUrl", skip_serializing_if = "Option::is_none")]
    pub icon_image_url: Option<String>,
    #[serde(rename = "coverImageUrl", skip_serializing_if = "Option::is_none")]
    pub cover_image_url: Option<String>,
    #[serde(rename = "primaryColorHex", skip_serializing_if = "Option::is_none")]
    pub primary_color_hex: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<crate::models::PublicPlayer>>,
}

impl GetProfileGames200ResponseInner {
    pub fn new() -> GetProfileGames200ResponseInner {
        GetProfileGames200ResponseInner {
            id: None,
            name: None,
            published_key: None,
            icon_image_url: None,
            cover_image_url: None,
            primary_color_hex: None,
            created_at: None,
            players: None,
        }
    }
}


