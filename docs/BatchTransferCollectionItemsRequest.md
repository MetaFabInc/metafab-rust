# BatchTransferCollectionItemsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | Option<**Vec<String>**> | An array of valid EVM based addresses to transfer items to. | [optional]
**wallet_ids** | Option<**Vec<String>**> | An array of wallet ids within the MetaFab ecosystem to transfer items to. | [optional]
**item_ids** | **Vec<f32>** | An array of unique itemIds to transfer. Each recipient will receive the same set of items provided. | 
**quantities** | **Vec<f32>** | The quantities of each unique itemId to transfer. Each recipient will receive the same quantities of items provided. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


