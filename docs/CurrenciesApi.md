# \CurrenciesApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_transfer_currency**](CurrenciesApi.md#batch_transfer_currency) | **POST** /v1/currencies/{currencyId}/batchTransfers | Batch transfer currency
[**burn_currency**](CurrenciesApi.md#burn_currency) | **POST** /v1/currencies/{currencyId}/burns | Burn currency
[**create_currency**](CurrenciesApi.md#create_currency) | **POST** /v1/currencies | Create currency
[**get_currencies**](CurrenciesApi.md#get_currencies) | **GET** /v1/currencies | Get currencies
[**get_currency_balance**](CurrenciesApi.md#get_currency_balance) | **GET** /v1/currencies/{currencyId}/balances | Get currency balance
[**get_currency_fees**](CurrenciesApi.md#get_currency_fees) | **GET** /v1/currencies/{currencyId}/fees | Get currency fees
[**get_currency_role**](CurrenciesApi.md#get_currency_role) | **GET** /v1/currencies/{currencyId}/roles | Get currency role
[**grant_currency_role**](CurrenciesApi.md#grant_currency_role) | **POST** /v1/currencies/{currencyId}/roles | Grant currency role
[**mint_currency**](CurrenciesApi.md#mint_currency) | **POST** /v1/currencies/{currencyId}/mints | Mint currency
[**revoke_currency_role**](CurrenciesApi.md#revoke_currency_role) | **DELETE** /v1/currencies/{currencyId}/roles | Revoke currency role
[**set_currency_fees**](CurrenciesApi.md#set_currency_fees) | **POST** /v1/currencies/{currencyId}/fees | Set currency fees
[**transfer_currency**](CurrenciesApi.md#transfer_currency) | **POST** /v1/currencies/{currencyId}/transfers | Transfer currency



## batch_transfer_currency

> crate::models::TransactionModel batch_transfer_currency(currency_id, x_authorization, x_wallet_decrypt_key, batch_transfer_currency_request)
Batch transfer currency

Transfers multiple amounts of currency to multiple provided wallet addresses or wallet addresses associated with the provided walletIds. You may also provide a combination of addresses and walletIds in one request, the proper receipients will be automatically determined, with `addresses` getting `amounts` order priority first.  Optional references may be included for the transfer. References are useful for identifying transfers intended to pay for items, trades, services and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**batch_transfer_currency_request** | [**BatchTransferCurrencyRequest**](BatchTransferCurrencyRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## burn_currency

> crate::models::TransactionModel burn_currency(currency_id, x_authorization, x_wallet_decrypt_key, burn_currency_request)
Burn currency

Removes (burns) the provided amount of currency from the authenticating game or players wallet. The currency amount is permanently removed from the circulating supply of the currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**burn_currency_request** | [**BurnCurrencyRequest**](BurnCurrencyRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_currency

> crate::models::CreateCurrency200Response create_currency(x_authorization, x_wallet_decrypt_key, create_currency_request)
Create currency

Creates a new game currency and deploys an ERC20 token contract on behalf of the authenticating game's primary wallet. The deployed ERC20 contract is preconfigured to fully support bridging across blockchains, batched transfers and gasless transactions on any supported blockchain as well as full support for gasless transactions from player managed wallets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**create_currency_request** | [**CreateCurrencyRequest**](CreateCurrencyRequest.md) |  | [required] |

### Return type

[**crate::models::CreateCurrency200Response**](createCurrency_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currencies

> Vec<crate::models::GetCurrencies200ResponseInner> get_currencies(x_game_key)
Get currencies

Returns an array of active currencies for the game associated with the provided `X-Game-Key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**Vec<crate::models::GetCurrencies200ResponseInner>**](getCurrencies_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currency_balance

> f32 get_currency_balance(currency_id, address, wallet_id)
Get currency balance

Returns the current currency balance of the provided wallet address or or the wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab platform. |  |

### Return type

**f32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currency_fees

> crate::models::GetCurrencyFees200Response get_currency_fees(currency_id)
Get currency fees

Returns the current fee recipient address and fees of the currency for the provided currencyId. Fees are only applicable for gasless transactions performed by default by players.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |

### Return type

[**crate::models::GetCurrencyFees200Response**](getCurrencyFees_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currency_role

> bool get_currency_role(currency_id, role, address, wallet_id)
Get currency role

Returns a boolean (true/false) representing if the provided role for this currency has been granted to the provided address or address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**role** | **String** | A valid MetaFab role or bytes string representing a role, such as `0xc9eb32e43bf5ecbceacf00b32281dfc5d6d700a0db676ea26ccf938a385ac3b7` | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab platform. |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_currency_role

> crate::models::TransactionModel grant_currency_role(currency_id, x_authorization, x_wallet_decrypt_key, grant_currency_role_request)
Grant currency role

Grants the provided role for the currency to the provided address or address associated with the provided walletId. Granted roles give different types of authority on behalf of the currency for specific players, addresses, or contracts to perform different types of permissioned currency operations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**grant_currency_role_request** | [**GrantCurrencyRoleRequest**](GrantCurrencyRoleRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_currency

> crate::models::TransactionModel mint_currency(currency_id, x_authorization, x_wallet_decrypt_key, mint_currency_request)
Mint currency

Creates (mints) the provided amount of currency to the provided wallet address or wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**mint_currency_request** | [**MintCurrencyRequest**](MintCurrencyRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_currency_role

> crate::models::TransactionModel revoke_currency_role(currency_id, x_authorization, x_wallet_decrypt_key, revoke_collection_role_request)
Revoke currency role

Revokes the provided role for the currency to the provided address or address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**revoke_collection_role_request** | [**RevokeCollectionRoleRequest**](RevokeCollectionRoleRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_currency_fees

> crate::models::TransactionModel set_currency_fees(currency_id, x_authorization, x_wallet_decrypt_key, set_currency_fees_request)
Set currency fees

Sets the recipient address, basis points, fixed amount and cap amount for a currency's fees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**set_currency_fees_request** | [**SetCurrencyFeesRequest**](SetCurrencyFeesRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_currency

> crate::models::TransactionModel transfer_currency(currency_id, x_authorization, x_wallet_decrypt_key, transfer_currency_request)
Transfer currency

Transfers an amount of currency to the provided wallet address or wallet address associated with the provided walletId. If you want to transfer to multiple wallets with different amounts and optional references in one API request, please see the Batch transfer currency documentation.  An optional reference may be included for the transfer. References are useful for identifying transfers intended to pay for items, trades, services and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_id** | **String** | Any currency id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**transfer_currency_request** | [**TransferCurrencyRequest**](TransferCurrencyRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

