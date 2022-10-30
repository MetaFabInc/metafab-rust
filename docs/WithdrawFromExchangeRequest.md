# WithdrawFromExchangeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | A valid EVM based address to withdraw to. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. | [optional]
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab ecosystem to withdraw to. | [optional]
**currency_address** | Option<**String**> | The address of the currency (ERC20) token to withdraw from the exchange. If no currencyAddress or currencyId, and no collectionAddress or collectionId are provided, the native token held by the exchange will be withdrawn. | [optional]
**currency_id** | Option<**String**> | A valid MetaFab currency id that represents the currency token to withdraw from the exchange. `currencyAddress` or `currencyId` can be provided when withdrawing currency. | [optional]
**collection_address** | Option<**String**> | The address of the collection (ERC1155) for the items to withdraw from the exchange. If no currencyAddress and no collectionAddress is provided, the native token held by the exchange will be withdrawn. | [optional]
**collection_id** | Option<**String**> | A valid MetaFab collection id that represents the collection for the items to withdraw from the exchange. `collectionAddress` or `collectionId` can be provided when withdrawing items. | [optional]
**item_ids** | Option<**Vec<f32>**> | The specific itemIds of the provided collection to withdraw from the exchange. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


