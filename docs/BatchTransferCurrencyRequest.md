# BatchTransferCurrencyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | Option<**Vec<String>**> | An array of valid EVM based addresses to transfer currency to. | [optional]
**wallet_ids** | Option<**Vec<String>**> | An array of wallet ids within the MetaFab ecosystem to transfer currency to. | [optional]
**amounts** | **Vec<f32>** | An array of currency amounts to transfer. Ordering corresponds to the ordering of provided `addresses` and/or `walletIds`. If both `addresses` and `walletIds` are provided, `addresses` are first in the order. | 
**references** | Option<**Vec<f32>**> | An optional array of uint256 numbers to reference each transfer in the batch. Ordering corresponds to the ordering of provided `addresses` or `walletIds`. If both `addresses` and `walletIds` are provided, `addresses` are first in the order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


