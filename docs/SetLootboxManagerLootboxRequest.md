# SetLootboxManagerLootboxRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | A unique lootbox id to use for this lootbox for the lootbox manager. If an existing lootbox id is used, the current lootbox will be updated but the existing number of opens will be kept. If you want to reset the number of opens for an existing lootbox, first remove it using the remove lootbox endpoint, then set it. | 
**input_collection_address** | Option<**String**> | A valid EVM based ERC1155 or MetaFab game items contract address that represents the collection for input items required by this lootbox. `inputCollectionAddress` or `inputCollectionId` can optionally be provided. | [optional]
**input_collection_id** | Option<**String**> | A valid MetaFab collection id that represents the collection for input items required by this lootbox. `inputCollectionAddress` or `inputCollectionId` can optionally be provided. | [optional]
**input_collection_item_ids** | Option<**Vec<i32>**> | An array of item ids from the provided input collection that are required to open this lootbox. Input items are burn upon opening a lootbox. | [optional]
**input_collection_item_amounts** | Option<**Vec<i32>**> | An array of amounts for each item id from the provided input collection that are required to open this lootbox. Item amounts array indices are reflective of the amount required for a given item id at the same index. | [optional]
**output_collection_address** | Option<**String**> | A valid EVM based ERC1155 or MetaFab game items contract address that represents the collection for possible output items given by this lootbox. `outputCollectionAddress` or `outputCollectionId` can optionally be provided. | [optional]
**output_collection_id** | Option<**String**> | A valid MetaFab collection id that represents the collection for possible output items given by this lootbox. `outputCollectionAddress` or `outputCollectionId` can optionally be provided. | [optional]
**output_collection_item_ids** | Option<**Vec<i32>**> | An array of item ids from the provided output collection that are possibly given by this lootbox. Randomly selected output items are automatically minted if the lootbox manager contract has the `minter` role for the output collection contract. Otherwise, they are transferred from the item balance held by the lootbox manager contract. | [optional]
**output_collection_item_amounts** | Option<**Vec<i32>**> | An array of amounts for each item id that can be randomly selected from the provided output collection that are given by this lootbox. Item amounts array indices are reflective of the amount required for a given item id at the same index. | [optional]
**output_collection_item_weights** | Option<**Vec<i32>**> | An array of weights for each item id that can be randomly selected from the provided output collection that are given by this lootbox. Any positive integer for an item's weight can be provided. The weight for an item relative to the sum of all possible item weights determines the probability that an item will be picked upon a lootbox being opened. Item weights array indices are reflective of the probability weight for a given item id at the same index. | [optional]
**output_total_items** | Option<**i32**> | The number of items randomly selected from the possible output items when this lootbox is open. If you provide a value greater than 1, it is possible for the same item to be selected more than once, giving the opener more than one of that item's output from the lootbox. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


