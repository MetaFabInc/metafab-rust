# ExchangeOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**f32**> | The id of this offer. | [optional]
**input_collection** | Option<**String**> | The address of the ERC1155 of MetaFab game items contract for input items required by this offer. | [optional]
**input_collection_item_ids** | Option<**Vec<f32>**> | An array of item ids from the input collection that are required for this offer. | [optional]
**input_collection_item_amounts** | Option<**Vec<f32>**> | An array of amounts for each item id for the input collection that are required to use this offer. | [optional]
**input_currency** | Option<**String**> | The address of the ERC20 or MetaFab game currency for the currency required by this offer. | [optional]
**input_currency_amount** | Option<**f32**> | The amount of currency required by this offer. | [optional]
**output_collection** | Option<**String**> | The address of the ERC1155 of MetaFab game items contract for output items given by this offer. | [optional]
**output_collection_item_ids** | Option<**Vec<f32>**> | An array of item ids from the output collection that are given for this offer. | [optional]
**output_collection_item_amounts** | Option<**Vec<f32>**> | An array of amounts for each item id for the output collection that are given by this offer. | [optional]
**output_currency** | Option<**String**> | The address of the ERC20 or MetaFab game currency for the output currency given by this offer. | [optional]
**output_currency_amount** | Option<**f32**> | The amount of currency given by this offer. | [optional]
**uses** | Option<**f32**> | The number of times this offer has been used. | [optional]
**max_uses** | Option<**f32**> | The maximum number of times this offer can be used. A value of `0` means there is no limit on how many times this offer can be used. | [optional]
**last_updated_at** | Option<**f32**> | A unix timestamp in seconds that represents the last time this offer was set or updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


