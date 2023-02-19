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
pub struct PublicEcosystem {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "publishedKey", skip_serializing_if = "Option::is_none")]
    pub published_key: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "iconImageUrl", skip_serializing_if = "Option::is_none")]
    pub icon_image_url: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "coverImageUrl", skip_serializing_if = "Option::is_none")]
    pub cover_image_url: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "primaryColorHex", skip_serializing_if = "Option::is_none")]
    pub primary_color_hex: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl PublicEcosystem {
    pub fn new() -> PublicEcosystem {
        PublicEcosystem {
            id: None,
            name: None,
            published_key: None,
            icon_image_url: None,
            cover_image_url: None,
            primary_color_hex: None,
            updated_at: None,
            created_at: None,
        }
    }
}

