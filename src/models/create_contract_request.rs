/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.2.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateContractRequest {
    /// The address of the existing contract.
    #[serde(rename = "address")]
    pub address: String,
    /// JSON of the abi.
    #[serde(rename = "abi")]
    pub abi: String,
    /// The blockchain you want to deploy this currency on. Support for new blockchains are added over time.
    #[serde(rename = "chain")]
    pub chain: Chain,
}

impl CreateContractRequest {
    pub fn new(address: String, abi: String, chain: Chain) -> CreateContractRequest {
        CreateContractRequest {
            address,
            abi,
            chain,
        }
    }
}

/// The blockchain you want to deploy this currency on. Support for new blockchains are added over time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Chain {
    #[serde(rename = "ETHEREUM")]
    Ethereum,
    #[serde(rename = "GOERLI")]
    Goerli,
    #[serde(rename = "MATIC")]
    Matic,
    #[serde(rename = "MATICMUMBAI")]
    Maticmumbai,
    #[serde(rename = "ARBITRUM")]
    Arbitrum,
    #[serde(rename = "ARBITRUMGOERLI")]
    Arbitrumgoerli,
}

impl Default for Chain {
    fn default() -> Chain {
        Self::Ethereum
    }
}

