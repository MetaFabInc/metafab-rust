# CreateCollectionItemRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | A unique itemId to use for this item within the collection. If an existing itemId is used, the current metadata will be overriden. Any number may be used.  The terms `itemId` and `collectionItemId` are used interchangeably and equivalent to one another throughout MetaFab documentation. | 
**name** | **String** | The name of this item. | 
**description** | **String** | A text description of this item. This is a great spot to include lore, game mechanics and more related to this item. | 
**image_base64** | Option<**String**> | A base64 string of the image for this item. Either `imageBase64` or `imageUrl` must be provided. Supported image formats are `jpg`, `jpeg`, `png`, `gif`. Recommended size of 1:1 aspect ratio and no more than 1500x1500 pixels. | [optional]
**image_url** | Option<**String**> | An external url that resolves to an image to use for this item. This can also be set to an ipfs:// uri. Recommended size of 1:1 aspect ratio and no more than 1500x1500 pixels. | [optional]
**external_url** | Option<**String**> | An optional URL where players can go to learn more about this item specifically, or your game, or any other link. | [optional]
**attributes** | Option<[**Vec<crate::models::CreateCollectionItemRequestAttributesInner>**](createCollectionItem_request_attributes_inner.md)> | An array of objects that conform with the [metadata attributes standard that can be found here](https://docs.opensea.io/docs/metadata-standards#attributes) | [optional]
**data** | Option<[**serde_json::Value**](.md)> | An arbitrary object of data attached to the top level metadata object. This is useful for including data or resource URLs specific to your game. Such as URLs that point to 3D models, music files, bitmaps, or anything else you need to reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


