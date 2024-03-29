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
pub struct UpgradeContractTrustedForwarderRequest {
    /// A ERC2771 forwarder smart contract address to assign as the new trusted forwarder of the target smart contract.
    #[serde(rename = "forwarderAddress")]
    pub forwarder_address: String,
}

impl UpgradeContractTrustedForwarderRequest {
    pub fn new(forwarder_address: String) -> UpgradeContractTrustedForwarderRequest {
        UpgradeContractTrustedForwarderRequest {
            forwarder_address,
        }
    }
}


