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
pub struct CreateCollection200ResponseAllOf {
    #[serde(rename = "contract", skip_serializing_if = "Option::is_none")]
    pub contract: Option<Box<crate::models::CreateCollection200ResponseAllOfContract>>,
}

impl CreateCollection200ResponseAllOf {
    pub fn new() -> CreateCollection200ResponseAllOf {
        CreateCollection200ResponseAllOf {
            contract: None,
        }
    }
}


