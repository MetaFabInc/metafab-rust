# LootboxManagerLootbox

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of this lootbox. | [optional]
**input_collection** | Option<**String**> | The address of the ERC1155 or MetaFab game items contract for input items required by this lootbox. | [optional]
**input_collection_item_ids** | Option<**Vec<i32>**> | An array of item ids from the input collection that are required for this lootbox. | [optional]
**input_collection_item_amounts** | Option<**Vec<i32>**> | An array of amounts for each item id for the input collection that are required to open this lootbox. | [optional]
**output_collection** | Option<**String**> | The address of the ERC1155 of MetaFab game items contract for possible output items given by this lootbox. | [optional]
**output_collection_item_ids** | Option<**Vec<i32>**> | An array of item ids from the output collection that are possibly given by this lootbox. | [optional]
**output_collection_item_amounts** | Option<**Vec<i32>**> | An array of amounts for each item id for the output collection that are possibly given by this lootbox. | [optional]
**output_collection_item_weights** | Option<**Vec<i32>**> | An array of weights for each item id for the output collection that are possibly given by this lootbox. | [optional]
**output_total_items** | Option<**i32**> | The number of items randomly selected when this lootbox is opened. | [optional]
**last_updated_at** | Option<**i32**> | A unix timestamp in seconds that represents the last time this offer was set or updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


