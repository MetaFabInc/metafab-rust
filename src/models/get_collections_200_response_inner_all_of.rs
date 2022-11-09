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
pub struct GetCollections200ResponseInnerAllOf {
    #[serde(rename = "contract", skip_serializing_if = "Option::is_none")]
    pub contract: Option<Box<crate::models::CollectionModel>>,
}

impl GetCollections200ResponseInnerAllOf {
    pub fn new() -> GetCollections200ResponseInnerAllOf {
        GetCollections200ResponseInnerAllOf {
            contract: None,
        }
    }
}


