# \LootboxesApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_lootbox_manager**](LootboxesApi.md#create_lootbox_manager) | **POST** /v1/lootboxManagers | Create lootbox manager
[**get_lootbox_manager_lootbox**](LootboxesApi.md#get_lootbox_manager_lootbox) | **GET** /v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId} | Get lootbox manager lootbox
[**get_lootbox_manager_lootboxes**](LootboxesApi.md#get_lootbox_manager_lootboxes) | **GET** /v1/lootboxManagers/{lootboxManagerId}/lootboxes | Get lootbox manager lootboxes
[**get_lootbox_managers**](LootboxesApi.md#get_lootbox_managers) | **GET** /v1/lootboxManagers | Get lootbox managers
[**open_lootbox_manager_lootbox**](LootboxesApi.md#open_lootbox_manager_lootbox) | **POST** /v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId}/opens | Open lootbox manager lootbox
[**remove_lootbox_manager_lootbox**](LootboxesApi.md#remove_lootbox_manager_lootbox) | **DELETE** /v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId} | Remove lootbox manager lootbox
[**set_lootbox_manager_lootbox**](LootboxesApi.md#set_lootbox_manager_lootbox) | **POST** /v1/lootboxManagers/{lootboxManagerId}/lootboxes | Set lootbox manager lootbox



## create_lootbox_manager

> crate::models::CreateLootboxManager200Response create_lootbox_manager(x_authorization, x_password, create_lootbox_manager_request)
Create lootbox manager

Creates a new game lootbox manager and deploys a lootbox manager contract on behalf of the authenticating game's primary wallet. The deployed lootbox manager contract allows you to create lootbox behavior for existing items. For example, you can define item id(s) from one of your item collections as the requirement(s) to open a \"lootbox\". The required item(s) would be burned from the interacting player's wallet and the player would receive item(s) from a weighted randomized set of possible items the lootbox can contain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**create_lootbox_manager_request** | [**CreateLootboxManagerRequest**](CreateLootboxManagerRequest.md) |  | [required] |

### Return type

[**crate::models::CreateLootboxManager200Response**](createLootboxManager_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lootbox_manager_lootbox

> crate::models::LootboxManagerLootbox get_lootbox_manager_lootbox(lootbox_manager_id, lootbox_manager_lootbox_id)
Get lootbox manager lootbox

Returns a lootbox manager lootbox object for the provided lootboxManagerLootboxId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lootbox_manager_id** | **String** | Any lootbox manager id within the MetaFab ecosystem. | [required] |
**lootbox_manager_lootbox_id** | **String** | Any lootbox manager lootbox id within the MetaFab ecosystem. | [required] |

### Return type

[**crate::models::LootboxManagerLootbox**](LootboxManagerLootbox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lootbox_manager_lootboxes

> Vec<crate::models::LootboxManagerLootbox> get_lootbox_manager_lootboxes(lootbox_manager_id)
Get lootbox manager lootboxes

Returns all lootbox manager lootboxes as an array of lootbox objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lootbox_manager_id** | **String** | Any lootbox manager id within the MetaFab ecosystem. | [required] |

### Return type

[**Vec<crate::models::LootboxManagerLootbox>**](LootboxManagerLootbox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lootbox_managers

> Vec<crate::models::GetLootboxManagers200ResponseInner> get_lootbox_managers(x_game_key)
Get lootbox managers

Returns an array of active lootbox managers for the game associated with the provided `X-Game-Key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**Vec<crate::models::GetLootboxManagers200ResponseInner>**](getLootboxManagers_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_lootbox_manager_lootbox

> Vec<crate::models::TransactionModel> open_lootbox_manager_lootbox(lootbox_manager_id, lootbox_manager_lootbox_id, x_authorization, x_password)
Open lootbox manager lootbox

Opens a lootbox manager lootbox. The required input item(s) are burned from the wallet or player wallet opening the lootbox. The given output item(s) are given to the wallet or player wallet opening the lootbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lootbox_manager_id** | **String** | Any lootbox manager id within the MetaFab ecosystem. | [required] |
**lootbox_manager_lootbox_id** | **String** | Any lootbox manager lootbox id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |

### Return type

[**Vec<crate::models::TransactionModel>**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_lootbox_manager_lootbox

> crate::models::TransactionModel remove_lootbox_manager_lootbox(lootbox_manager_id, lootbox_manager_lootbox_id, x_authorization, x_password)
Remove lootbox manager lootbox

Removes the provided lootbox by lootboxId from the provided lootbox manager. Removed lootboxes can no longer be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lootbox_manager_id** | **String** | Any lootbox manager id within the MetaFab ecosystem. | [required] |
**lootbox_manager_lootbox_id** | **String** | Any lootbox manager lootbox id within the MetaFab ecosystem. | [required] |
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


## set_lootbox_manager_lootbox

> crate::models::TransactionModel set_lootbox_manager_lootbox(lootbox_manager_id, x_authorization, x_password, set_lootbox_manager_lootbox_request)
Set lootbox manager lootbox

Sets a new lootbox manager lootbox or updates an existing one for the provided id. Lootboxes allow item(s) to be burned to receive a random set of possible item(s) based on probability weight.  Lootboxes can require any number of unique types of items and quantities to open a created lootbox type within the system. A common pattern with lootboxes is to create a lootbox item type within an item collection, and require it as the input item type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lootbox_manager_id** | **String** | Any lootbox manager id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**set_lootbox_manager_lootbox_request** | [**SetLootboxManagerLootboxRequest**](SetLootboxManagerLootboxRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

