/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BurnCollectionItemRequest {
    /// The quantity of the collectionItemId to burn.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl BurnCollectionItemRequest {
    pub fn new(quantity: i32) -> BurnCollectionItemRequest {
        BurnCollectionItemRequest {
            quantity,
        }
    }
}


