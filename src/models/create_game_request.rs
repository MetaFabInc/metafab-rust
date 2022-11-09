/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.2.1
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateGameRequest {
    /// The name of the game you're creating.
    #[serde(rename = "name")]
    pub name: String,
    /// The email address associated with this game and used to login/authenticate as the game.
    #[serde(rename = "email")]
    pub email: String,
    /// The password to authenticate as the game. Additionally, this password is used to encrypt/decrypt your game's primary wallet and must be provided anytime this game makes blockchain interactions through various endpoints.
    #[serde(rename = "password")]
    pub password: String,
}

impl CreateGameRequest {
    pub fn new(name: String, email: String, password: String) -> CreateGameRequest {
        CreateGameRequest {
            name,
            email,
            password,
        }
    }
}


