# SetShopOfferRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | A unique offer id to use for this offer for the shop. If an existing offer id is used, the current offer will be updated but the existing number of uses will be kept. If you want to reset the number of uses for an existing offer, first remove it using the remove offer endpoint, then set it. | 
**input_collection_address** | Option<**String**> | A valid EVM based ERC1155 or MetaFab game items contract address that represents the collection for input items required by this offer. `inputCollectionAddress` or `inputCollectionId` can optionally be provided. | [optional]
**input_collection_id** | Option<**String**> | A valid MetaFab collection id that represents the collection for input items required by this offer. `inputCollectionAddress` or `inputCollectionId` can optionally be provided. | [optional]
**input_collection_item_ids** | Option<**Vec<i32>**> | An array of item ids from the provided input collection that are required to use this offer. Input items are transferred from the wallet to the shop contract upon using an offer. | [optional]
**input_collection_item_amounts** | Option<**Vec<i32>**> | An array of amounts for each item id from the provided input collection that are required to use this offer. Item amounts array indices are reflective of the amount required for a given item id at the same index. | [optional]
**input_currency_address** | Option<**String**> | A valid EVM based ERC20 or MetaFab game currency contract address that for the currency required by this offer. | [optional]
**input_currency_id** | Option<**String**> | A valid MetaFab currency id that represents the currency required by this offer. | [optional]
**input_currency_amount** | Option<**f32**> | The amount of currency required by this offer. If an inputCurrencyAmount is provided without in input currency address or id, the native chain currency is used as the required currency. | [optional]
**output_collection_address** | Option<**String**> | A valid EVM based ERC1155 or MetaFab game items contract address that represents the collection for output items given by this offer. `outputCollectionAddress` or `outputCollectionId` can optionally be provided. | [optional]
**output_collection_id** | Option<**String**> | A valid MetaFab collection id that represents the collection for output items given by this offer. `outputCollectionAddress` or `outputCollectionId` can optionally be provided. | [optional]
**output_collection_item_ids** | Option<**Vec<i32>**> | An array of item ids from the provided output collection that are given by this offer. Output items are automatically minted if the shop contract has the `minter` role for the output collection contract. Otherwise, they are transferred from the item balance held by the shop contract. | [optional]
**output_collection_item_amounts** | Option<**Vec<i32>**> | An array of amounts for each item id from the provided output collection that are given by this offer. Item amounts array indices are reflective of the amount required for a given item id at the same index. | [optional]
**output_currency_address** | Option<**String**> | A valid EVM based ERC20 or MetaFab game currency contract address that for the currency given by this offer. The output currency amount is automatically minted if the shop contract has the `minter` role for the output currency contract. Otherwise, they are transferred from the currency balance held by the shop contract. | [optional]
**output_currency_id** | Option<**String**> | A valid MetaFab currency id for the currency given by this offer. | [optional]
**output_currency_amount** | Option<**f32**> | The amount of currency given by this offer. If an outputCurrencyAmount is provided without an output currency address or id, the native chain currency is used as the given currency. | [optional]
**max_uses** | Option<**i32**> | The maximum number of times this offer can be used in total. maxUses is collective across all uses of the offer. If 5 unique players use an offer, that counts as 5 offer uses. Exclude this or use 0 to allow unlimited uses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


