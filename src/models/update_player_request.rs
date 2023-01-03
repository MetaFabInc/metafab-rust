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
pub struct UpdatePlayerRequest {
    /// The player's current password. Must be provided if setting `newPassword`.
    #[serde(rename = "currentPassword", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// A new password. The player's old password will no longer be valid.
    #[serde(rename = "newPassword", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    /// Revokes the player's previous access token and returns a new one if true.
    #[serde(rename = "resetAccessToken", skip_serializing_if = "Option::is_none")]
    pub reset_access_token: Option<bool>,
}

impl UpdatePlayerRequest {
    pub fn new() -> UpdatePlayerRequest {
        UpdatePlayerRequest {
            current_password: None,
            new_password: None,
            reset_access_token: None,
        }
    }
}


