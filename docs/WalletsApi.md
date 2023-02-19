# \WalletsApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_wallet_signature**](WalletsApi.md#create_wallet_signature) | **POST** /v1/wallets/{walletId}/signatures | Create wallet signature
[**get_wallet**](WalletsApi.md#get_wallet) | **GET** /v1/wallets/{walletId} | Get wallet
[**get_wallet_balances**](WalletsApi.md#get_wallet_balances) | **GET** /v1/wallets/{walletId}/balances | Get wallet balances
[**get_wallet_transactions**](WalletsApi.md#get_wallet_transactions) | **GET** /v1/wallets/{walletId}/transactions | Get wallet transactions



## create_wallet_signature

> String create_wallet_signature(wallet_id, x_wallet_decrypt_key, create_wallet_signature_request)
Create wallet signature

Creates a wallet signature from a plaintext message using the wallet for the provided walletId and walletDecryptKey. Wallet signatures cannot be generated for EOA wallets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | Any wallet id within the MetaFab platform. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` for the provided `walletId`. | [required] |
**create_wallet_signature_request** | [**CreateWalletSignatureRequest**](CreateWalletSignatureRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet

> crate::models::WalletModel get_wallet(wallet_id)
Get wallet

Returns a wallet object for the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | Any wallet id within the MetaFab platform. | [required] |

### Return type

[**crate::models::WalletModel**](WalletModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet_balances

> ::std::collections::HashMap<String, f32> get_wallet_balances(wallet_id)
Get wallet balances

Returns the current native token balance for all chains supported by MetaFab for the provided walletId. This includes balances like Eth, Matic and other native tokens from chains MetaFab supports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | Any wallet id within the MetaFab platform. | [required] |

### Return type

**::std::collections::HashMap<String, f32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet_transactions

> Vec<crate::models::TransactionModel> get_wallet_transactions(wallet_id)
Get wallet transactions

Returns an array of MetaFab initiated transactions performed by the provided walletId. Transactions returned are ordered chronologically from newest to oldest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | **String** | Any wallet id within the MetaFab platform. | [required] |

### Return type

[**Vec<crate::models::TransactionModel>**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

