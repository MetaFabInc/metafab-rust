/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCollectionItemRequestAttributesInner {
    #[serde(rename = "trait_type")]
    pub trait_type: String,
    #[serde(rename = "value")]
    pub value: Box<crate::models::CreateCollectionItemRequestAttributesInnerValue>,
}

impl CreateCollectionItemRequestAttributesInner {
    pub fn new(trait_type: String, value: crate::models::CreateCollectionItemRequestAttributesInnerValue) -> CreateCollectionItemRequestAttributesInner {
        CreateCollectionItemRequestAttributesInner {
            trait_type,
            value: Box::new(value),
        }
    }
}


