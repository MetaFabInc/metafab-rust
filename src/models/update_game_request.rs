/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateGameRequest {
    /// A new name. Replaces the game's current name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A new email address. The game's old email will no longer be valid for account authentication. `currentPassword` must also be provided.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The game's current password. Must be provided if setting `newPassword` or `email`.
    #[serde(rename = "currentPassword", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// A new password. The game's old password will no longer be valid.
    #[serde(rename = "newPassword", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    /// Sets a custom RPC for your game to use instead of MetaFab's default RPCs for the chain(s) you specify.  Expects a JSON object containing key value pairs of supported `chain` -> `rpc url`. Only the chain names provided as keys in the object will be explicitly overriden. To delete a custom RPC for your game, provide the chain name to delete as a key in the provided object and `null` as the value.  Set RPC example, `{ MATIC: 'https://polygon-rpc.com' }` Delete RPC example, `{ MATIC: null }`
    #[serde(rename = "rpcs", skip_serializing_if = "Option::is_none")]
    pub rpcs: Option<::std::collections::HashMap<String, String>>,
    /// Revokes the game's previous published key and returns a new one if true.
    #[serde(rename = "resetPublishedKey", skip_serializing_if = "Option::is_none")]
    pub reset_published_key: Option<bool>,
    /// Revokes the game's previous secret key and returns a new on if true.
    #[serde(rename = "resetSecretKey", skip_serializing_if = "Option::is_none")]
    pub reset_secret_key: Option<bool>,
}

impl UpdateGameRequest {
    pub fn new() -> UpdateGameRequest {
        UpdateGameRequest {
            name: None,
            email: None,
            current_password: None,
            new_password: None,
            rpcs: None,
            reset_published_key: None,
            reset_secret_key: None,
        }
    }
}


