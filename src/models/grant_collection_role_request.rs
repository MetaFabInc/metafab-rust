/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.43
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GrantCollectionRoleRequest {
    /// A valid MetaFab role or bytes string representing a role, such as `minter` or `0xc9eb32e43bf5ecbceacf00b32281dfc5d6d700a0db676ea26ccf938a385ac3b7`
    #[serde(rename = "role")]
    pub role: String,
    /// A valid EVM based address grant the role to.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// A wallet id within the MetaFab ecosystem to grant the role to.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<Vec<String>>,
}

impl GrantCollectionRoleRequest {
    pub fn new(role: String) -> GrantCollectionRoleRequest {
        GrantCollectionRoleRequest {
            role,
            address: None,
            wallet_id: None,
        }
    }
}


