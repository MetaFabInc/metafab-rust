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
pub struct CreateCollection200ResponseAllOfContractAllOf {
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::TransactionModel>>,
}

impl CreateCollection200ResponseAllOfContractAllOf {
    pub fn new() -> CreateCollection200ResponseAllOfContractAllOf {
        CreateCollection200ResponseAllOfContractAllOf {
            transactions: None,
        }
    }
}


