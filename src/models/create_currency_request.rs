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
pub struct CreateCurrencyRequest {
    /// The name of this currency. This can be anything, such as `Bright Gems`, `Gold`, etc.
    #[serde(rename = "name")]
    pub name: String,
    /// The shorthand symbol to represent this currency. This can be anything, such as `BGEM`, `GLD`, etc.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The maximum amount of this currency that can ever exist. Use `0` if you do not want this currency to have a maximum supply.
    #[serde(rename = "supplyCap")]
    pub supply_cap: f32,
    /// The blockchain you want to deploy this currency on. Support for new blockchains are added over time.
    #[serde(rename = "chain")]
    pub chain: Chain,
}

impl CreateCurrencyRequest {
    pub fn new(name: String, symbol: String, supply_cap: f32, chain: Chain) -> CreateCurrencyRequest {
        CreateCurrencyRequest {
            name,
            symbol,
            supply_cap,
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
    #[serde(rename = "MUMBAI")]
    Mumbai,
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

