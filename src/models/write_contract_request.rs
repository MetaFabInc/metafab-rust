/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.3
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WriteContractRequest {
    /// A contract function name. This can be any valid function from the the ABI of the contract you are interacting with. For example, `mint`.
    #[serde(rename = "func")]
    pub func: String,
    /// An array of args. This is optional and only necessary if the function being invoked requires arguments per the contract ABI. For example, `[123, \"Hello\", false]`.
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
}

impl WriteContractRequest {
    pub fn new(func: String) -> WriteContractRequest {
        WriteContractRequest {
            func,
            args: None,
        }
    }
}


