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
pub struct AuthGame200Response {
    /// This field has not had a description added.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "fundingWalletId", skip_serializing_if = "Option::is_none")]
    pub funding_wallet_id: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "rpcs", skip_serializing_if = "Option::is_none")]
    pub rpcs: Option<serde_json::Value>,
    /// This field has not had a description added.
    #[serde(rename = "redirectUris", skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<serde_json::Value>,
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
    #[serde(rename = "publishedKey", skip_serializing_if = "Option::is_none")]
    pub published_key: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// This field has not had a description added.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// This field has not had a description added.
    #[serde(rename = "walletDecryptKey", skip_serializing_if = "Option::is_none")]
    pub wallet_decrypt_key: Option<String>,
    #[serde(rename = "wallet", skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Box<crate::models::WalletModel>>,
    #[serde(rename = "fundingWallet", skip_serializing_if = "Option::is_none")]
    pub funding_wallet: Option<Box<crate::models::WalletModel>>,
}

impl AuthGame200Response {
    pub fn new() -> AuthGame200Response {
        AuthGame200Response {
            id: None,
            wallet_id: None,
            funding_wallet_id: None,
            email: None,
            name: None,
            rpcs: None,
            redirect_uris: None,
            icon_image_url: None,
            cover_image_url: None,
            primary_color_hex: None,
            published_key: None,
            secret_key: None,
            verified: None,
            updated_at: None,
            created_at: None,
            wallet_decrypt_key: None,
            wallet: None,
            funding_wallet: None,
        }
    }
}


