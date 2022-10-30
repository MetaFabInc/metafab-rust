# \ExchangesApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_exchange**](ExchangesApi.md#create_exchange) | **POST** /v1/exchanges | Create exchange
[**get_exchange_offer**](ExchangesApi.md#get_exchange_offer) | **GET** /v1/exchanges/{exchangeId}/items/{exchangeOfferId} | Get exchange offer
[**get_exchange_offers**](ExchangesApi.md#get_exchange_offers) | **GET** /v1/exchanges/{exchangeId}/offers | Get exchange offers
[**get_exchanges**](ExchangesApi.md#get_exchanges) | **GET** /v1/exchanges | Get exchanges
[**remove_exchange_offer**](ExchangesApi.md#remove_exchange_offer) | **DELETE** /v1/exchanges/{exchangeId}/offers/{exchangeOfferId} | Remove exchange offer
[**set_exchange_offer**](ExchangesApi.md#set_exchange_offer) | **POST** /v1/exchanges/{exchangeId}/offers | Set exchange offer
[**use_exchange_offer**](ExchangesApi.md#use_exchange_offer) | **POST** /v1/exchanges/{exchangeId}/offers/{exchangeOfferId}/uses | Use exchange offer
[**withdraw_from_exchange**](ExchangesApi.md#withdraw_from_exchange) | **POST** /v1/exchanges/{exchangeId}/withdrawals | Withdraw from exchange



## create_exchange

> crate::models::CreateExchange200Response create_exchange(x_authorization, x_password, create_exchange_request)
Create exchange

Creates a new game exchange and deploys a exchange contract on behalf of the authenticating game's primary wallet. The deployed exchange contract allows you to create fixed price rates for players to buy specific items from any item collection or ERC1155 contract. Additionally, an exchange allows you to create exchange offers for some set of item(s) to another set of item(s) or any mix of currency. Exchanges completely supports gasless player transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**create_exchange_request** | [**CreateExchangeRequest**](CreateExchangeRequest.md) |  | [required] |

### Return type

[**crate::models::CreateExchange200Response**](createExchange_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_offer

> crate::models::ExchangeOffer get_exchange_offer(exchange_id, exchange_offer_id)
Get exchange offer

Returns a exchange offer object for the provided exchangeOfferId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_id** | **String** | Any exchange id within the MetaFab ecosystem. | [required] |
**exchange_offer_id** | **String** | Any offer id for the exchange. Zero, or a positive integer. | [required] |

### Return type

[**crate::models::ExchangeOffer**](ExchangeOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_offers

> Vec<crate::models::ExchangeOffer> get_exchange_offers(exchange_id)
Get exchange offers

Returns all exchange offers as an array of exchange offer objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_id** | **String** | Any exchange id within the MetaFab ecosystem. | [required] |

### Return type

[**Vec<crate::models::ExchangeOffer>**](ExchangeOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchanges

> Vec<crate::models::GetExchanges200ResponseInner> get_exchanges(x_game_key)
Get exchanges

Returns an array of active exchanges for the game associated with the provided `X-Game-Key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**Vec<crate::models::GetExchanges200ResponseInner>**](getExchanges_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_exchange_offer

> crate::models::TransactionModel remove_exchange_offer(exchange_id, exchange_offer_id, x_authorization, x_password)
Remove exchange offer

Removes the provided offerId from the provided exchange. Removed offers can no longer be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_id** | **String** | Any exchange id within the MetaFab ecosystem. | [required] |
**exchange_offer_id** | **String** | Any offer id for the exchange. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_exchange_offer

> crate::models::TransactionModel set_exchange_offer(exchange_id, x_authorization, x_password, set_exchange_offer_request)
Set exchange offer

Sets a new exchange offer or updates an existing one for the provided id. Exchange offers allow currency to item, item to currency or item to item exchanges.  All request fields besides `id` are optional. Any optional fields omitted will not be used for the offer. This allows you to create many different combinations of offers. For example, you can create an offer that may require 3 unique item ids of specified quantities from a given item collection and gives the user 1 new unique item id in exchange.  Another example, you may want to make an exchange offer from one ERC20 token to another. This is also possible - simple set the input and output currency fields and leave the others blank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_id** | **String** | Any exchange id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**set_exchange_offer_request** | [**SetExchangeOfferRequest**](SetExchangeOfferRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## use_exchange_offer

> crate::models::TransactionModel use_exchange_offer(exchange_id, exchange_offer_id, x_authorization, x_password)
Use exchange offer

Uses an exchange offer. The required (input) item(s) and/or currency are removed from the wallet or player wallet using the offer. The given (output) item(s) and/or currency are given to the wallet or player wallet using the offer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_id** | **String** | Any exchange id within the MetaFab ecosystem. | [required] |
**exchange_offer_id** | **String** | Any offer id for the exchange. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_from_exchange

> crate::models::TransactionModel withdraw_from_exchange(exchange_id, x_authorization, x_password, withdraw_from_exchange_request)
Withdraw from exchange

Withdraws native token, currency or items from a exchange. Whenever an exchange offer has input requirements, the native tokens, currencies or items for the requirements of that offer are deposited into the exchange contract when the offer is used. These can be withdrawn to any other address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_id** | **String** | Any exchange id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**withdraw_from_exchange_request** | [**WithdrawFromExchangeRequest**](WithdrawFromExchangeRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

