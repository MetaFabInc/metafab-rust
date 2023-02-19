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
pub struct CreateEcosystemRequest {
    /// The name of the ecosystem you're creating.
    #[serde(rename = "name")]
    pub name: String,
    /// The email address associated with this ecosystem and used to login/authenticate as the ecosystem.
    #[serde(rename = "email")]
    pub email: String,
    /// The password to authenticate as the ecosystem.
    #[serde(rename = "password")]
    pub password: String,
}

impl CreateEcosystemRequest {
    pub fn new(name: String, email: String, password: String) -> CreateEcosystemRequest {
        CreateEcosystemRequest {
            name,
            email,
            password,
        }
    }
}


