# \ShopsApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_shop**](ShopsApi.md#create_shop) | **POST** /v1/shops | Create shop
[**get_shop_offer**](ShopsApi.md#get_shop_offer) | **GET** /v1/shops/{shopId}/offers/{shopOfferId} | Get shop offer
[**get_shop_offers**](ShopsApi.md#get_shop_offers) | **GET** /v1/shops/{shopId}/offers | Get shop offers
[**get_shops**](ShopsApi.md#get_shops) | **GET** /v1/shops | Get shops
[**remove_shop_offer**](ShopsApi.md#remove_shop_offer) | **DELETE** /v1/shops/{shopId}/offers/{shopOfferId} | Remove shop offer
[**set_shop_offer**](ShopsApi.md#set_shop_offer) | **POST** /v1/shops/{shopId}/offers | Set shop offer
[**use_shop_offer**](ShopsApi.md#use_shop_offer) | **POST** /v1/shops/{shopId}/offers/{shopOfferId}/uses | Use shop offer
[**withdraw_from_shop**](ShopsApi.md#withdraw_from_shop) | **POST** /v1/shops/{shopId}/withdrawals | Withdraw from shop



## create_shop

> crate::models::CreateShop200Response create_shop(x_authorization, x_wallet_decrypt_key, create_shop_request)
Create shop

Creates a new game shop and deploys a shop contract on behalf of the authenticating game's primary wallet. The deployed shop contract allows you to create fixed price rates for players to buy specific items from any item collection or ERC1155 contract. Additionally, a shop allows you to create shop offers for some set of item(s) to another set of item(s) or any mix of currency. Shops completely support gasless player transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**create_shop_request** | [**CreateShopRequest**](CreateShopRequest.md) |  | [required] |

### Return type

[**crate::models::CreateShop200Response**](createShop_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shop_offer

> crate::models::ShopOffer get_shop_offer(shop_id, shop_offer_id)
Get shop offer

Returns a shop offer object for the provided shopOfferId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shop_id** | **String** | Any shop id within the MetaFab platform. | [required] |
**shop_offer_id** | **String** | Any offer id for the shop. Zero, or a positive integer. | [required] |

### Return type

[**crate::models::ShopOffer**](ShopOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shop_offers

> Vec<crate::models::ShopOffer> get_shop_offers(shop_id)
Get shop offers

Returns all shop offers as an array of shop offer objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shop_id** | **String** | Any shop id within the MetaFab platform. | [required] |

### Return type

[**Vec<crate::models::ShopOffer>**](ShopOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shops

> Vec<crate::models::GetShops200ResponseInner> get_shops(x_game_key)
Get shops

Returns an array of active shops for the game associated with the provided `X-Game-Key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**Vec<crate::models::GetShops200ResponseInner>**](getShops_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_shop_offer

> crate::models::TransactionModel remove_shop_offer(shop_id, shop_offer_id, x_authorization, x_wallet_decrypt_key)
Remove shop offer

Removes the provided offer by offerId from the provided shop. Removed offers can no longer be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shop_id** | **String** | Any shop id within the MetaFab platform. | [required] |
**shop_offer_id** | **String** | Any offer id for the shop. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_shop_offer

> crate::models::TransactionModel set_shop_offer(shop_id, x_authorization, x_wallet_decrypt_key, set_shop_offer_request)
Set shop offer

Sets a new shop offer or updates an existing one for the provided id. Shop offers allow currency to item, item to currency or item to item exchanges.  All request fields besides `id` are optional. Any optional fields omitted will not be used for the offer. This allows you to create many different combinations of offers. For example, you can create an offer that may require 3 unique item ids of specified quantities from a given item collection and gives the user 1 new unique item id in exchange.  Another example, you may want to make a shop offer from one ERC20 token to another. This is also possible - simple set the input and output currency fields and leave the others blank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shop_id** | **String** | Any shop id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**set_shop_offer_request** | [**SetShopOfferRequest**](SetShopOfferRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## use_shop_offer

> crate::models::TransactionModel use_shop_offer(shop_id, shop_offer_id, x_authorization, x_wallet_decrypt_key)
Use shop offer

Uses a shop offer. The required (input) item(s) and/or currency are removed from the wallet or player wallet using the offer. The given (output) item(s) and/or currency are given to the wallet or player wallet using the offer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shop_id** | **String** | Any shop id within the MetaFab platform. | [required] |
**shop_offer_id** | **String** | Any offer id for the shop. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_from_shop

> crate::models::TransactionModel withdraw_from_shop(shop_id, x_authorization, x_wallet_decrypt_key, withdraw_from_shop_request)
Withdraw from shop

Withdraws native token, currency or items from a shop. Whenever a shop offer has input requirements, the native tokens, currencies or items for the requirements of that offer are deposited into the shop contract when the offer is used. These can be withdrawn to any other address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shop_id** | **String** | Any shop id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**withdraw_from_shop_request** | [**WithdrawFromShopRequest**](WithdrawFromShopRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

