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
pub struct CreateWalletSignatureRequest {
    /// The plaintext message to sign.
    #[serde(rename = "message")]
    pub message: String,
}

impl CreateWalletSignatureRequest {
    pub fn new(message: String) -> CreateWalletSignatureRequest {
        CreateWalletSignatureRequest {
            message,
        }
    }
}

