# \ItemsApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_mint_collection_items**](ItemsApi.md#batch_mint_collection_items) | **POST** /v1/collections/{collectionId}/batchMints | Batch mint collection items
[**batch_transfer_collection_items**](ItemsApi.md#batch_transfer_collection_items) | **POST** /v1/collections/{collectionId}/batchTransfers | Batch transfer collection items
[**burn_collection_item**](ItemsApi.md#burn_collection_item) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/burns | Burn collection item
[**create_collection**](ItemsApi.md#create_collection) | **POST** /v1/collections | Create collection
[**create_collection_item**](ItemsApi.md#create_collection_item) | **POST** /v1/collections/{collectionId}/items | Create collection item
[**get_collection_approval**](ItemsApi.md#get_collection_approval) | **GET** /v1/collections/{collectionId}/approvals | Get collection approval
[**get_collection_item**](ItemsApi.md#get_collection_item) | **GET** /v1/collections/{collectionId}/items/{collectionItemId} | Get collection item
[**get_collection_item_balance**](ItemsApi.md#get_collection_item_balance) | **GET** /v1/collections/{collectionId}/items/{collectionItemId}/balances | Get collection item balance
[**get_collection_item_balances**](ItemsApi.md#get_collection_item_balances) | **GET** /v1/collections/{collectionId}/balances | Get collection item balances
[**get_collection_item_supplies**](ItemsApi.md#get_collection_item_supplies) | **GET** /v1/collections/{collectionId}/supplies | Get collection item supplies
[**get_collection_item_supply**](ItemsApi.md#get_collection_item_supply) | **GET** /v1/collections/{collectionId}/items/{collectionItemId}/supplies | Get collection item supply
[**get_collection_item_timelock**](ItemsApi.md#get_collection_item_timelock) | **GET** /v1/collections/{collectionId}/items/{collectionItemId}/timelocks | Get collection item timelock
[**get_collection_items**](ItemsApi.md#get_collection_items) | **GET** /v1/collections/{collectionId}/items | Get collection items
[**get_collection_role**](ItemsApi.md#get_collection_role) | **GET** /v1/collections/{collectionId}/roles | Get collection role
[**get_collections**](ItemsApi.md#get_collections) | **GET** /v1/collections | Get collections
[**grant_collection_role**](ItemsApi.md#grant_collection_role) | **POST** /v1/collections/{collectionId}/roles | Grant collection role
[**mint_collection_item**](ItemsApi.md#mint_collection_item) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/mints | Mint collection item
[**revoke_collection_role**](ItemsApi.md#revoke_collection_role) | **DELETE** /v1/collections/{collectionId}/roles | Revoke collection role
[**set_collection_approval**](ItemsApi.md#set_collection_approval) | **POST** /v1/collections/{collectionId}/approvals | Set collection approval
[**set_collection_item_timelock**](ItemsApi.md#set_collection_item_timelock) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/timelocks | Set collection item timelock
[**transfer_collection_item**](ItemsApi.md#transfer_collection_item) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/transfers | Transfer collection item



## batch_mint_collection_items

> crate::models::TransactionModel batch_mint_collection_items(collection_id, x_authorization, x_password, batch_mint_collection_items_request)
Batch mint collection items

Creates (mints) the provided itemIds of the specified quantities to the provided wallet address or wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**batch_mint_collection_items_request** | [**BatchMintCollectionItemsRequest**](BatchMintCollectionItemsRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_transfer_collection_items

> crate::models::TransactionModel batch_transfer_collection_items(collection_id, x_authorization, x_password, batch_transfer_collection_items_request)
Batch transfer collection items

Transfers one or multiple items of specified quantities to the provided wallet addresses or wallet addresses associated with the provided walletIds. You may also provide a combination of addresses and walletIds in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**batch_transfer_collection_items_request** | [**BatchTransferCollectionItemsRequest**](BatchTransferCollectionItemsRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## burn_collection_item

> crate::models::TransactionModel burn_collection_item(collection_id, collection_item_id, x_authorization, x_password, burn_collection_item_request)
Burn collection item

Removes (burns) the provided quantity of the collectionItemId from the authenticating game or players wallet. The quantity is permanently removed from the circulating supply of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**burn_collection_item_request** | [**BurnCollectionItemRequest**](BurnCollectionItemRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_collection

> crate::models::CreateCollection200Response create_collection(x_authorization, x_password, create_collection_request)
Create collection

Creates a new game item collection and deploys an extended functionality ERC1155 contract on behalf of the authenticating game's primary wallet. The deployed ERC1155 contract is preconfigured to fully support creating unique item types, item transfer timelocks, custom metadata per item, gasless transactions from player managed wallets, and much more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**create_collection_request** | [**CreateCollectionRequest**](CreateCollectionRequest.md) |  | [required] |

### Return type

[**crate::models::CreateCollection200Response**](createCollection_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_collection_item

> crate::models::TransactionModel create_collection_item(collection_id, x_authorization, x_password, create_collection_item_request)
Create collection item

Creates a new item type. Item type creation associates all of the relevant item data to a specific itemId. Such as item name, image, description, attributes, any arbitrary data such as 2D or 3D model resolver URLs, and more. It is recommended, but not required, that you create a new item type through this endpoint before minting any quantity of the related itemId.  Any itemId provided will have its existing item type overriden if it already exists.  Item type data is uploaded to, and resolved through IPFS for decentralized persistence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**create_collection_item_request** | [**CreateCollectionItemRequest**](CreateCollectionItemRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_approval

> bool get_collection_approval(collection_id, operator_address, address, wallet_id)
Get collection approval

Returns a boolean (true/false) representing if the provided operatorAddress has approval to transfer and burn items from the current collection owned by the address or address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**operator_address** | **String** | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab ecosystem. |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_item

> serde_json::Value get_collection_item(collection_id, collection_item_id)
Get collection item

Returns a metadata object for the provided collectionItemId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_item_balance

> f32 get_collection_item_balance(collection_id, collection_item_id, address, wallet_id)
Get collection item balance

Returns the current collection item balance of the provided collectionItemId for the provided wallet address or the wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab ecosystem. |  |

### Return type

**f32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_item_balances

> ::std::collections::HashMap<String, f32> get_collection_item_balances(collection_id, address, wallet_id)
Get collection item balances

Returns the current collection item balances of all collection items for the provided wallet address or the wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab ecosystem. |  |

### Return type

**::std::collections::HashMap<String, f32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_item_supplies

> ::std::collections::HashMap<String, f32> get_collection_item_supplies(collection_id)
Get collection item supplies

Returns the currency circulating supply of all collection items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |

### Return type

**::std::collections::HashMap<String, f32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_item_supply

> f32 get_collection_item_supply(collection_id, collection_item_id, address, wallet_id)
Get collection item supply

Returns the current circulating supply of the provided collectionItemId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab ecosystem. |  |

### Return type

**f32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_item_timelock

> f32 get_collection_item_timelock(collection_id, collection_item_id)
Get collection item timelock

Returns a timestamp (in seconds) for when the provided collectionItemId's transfer timelock expires. A value of 0 means the provided collectionItemId does not have a timelock set. Timelocks prevent items of a specific collectionItemId from being transferred until the set timelock timestamp has been surpassed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |

### Return type

**f32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_items

> Vec<serde_json::Value> get_collection_items(collection_id)
Get collection items

Returns all collection items as an array of metadata objects.  Please note that ONLY items that have had at least 1 quantity minted will be returned. If you've created an item that has not been minted yet, it will not be returned in the array response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_role

> bool get_collection_role(collection_id, role, address, wallet_id)
Get collection role

Returns a boolean (true/false) representing if the provided role for this collection has been granted to the provided address or address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**role** | **String** | A valid MetaFab role or bytes string representing a role, such as `0xc9eb32e43bf5ecbceacf00b32281dfc5d6d700a0db676ea26ccf938a385ac3b7` | [required] |
**address** | Option<**String**> | A valid EVM based address. For example, `0x39cb70F972E0EE920088AeF97Dbe5c6251a9c25D`. |  |
**wallet_id** | Option<**String**> | Any wallet id within the MetaFab ecosystem. |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collections

> Vec<crate::models::GetCollections200ResponseInner> get_collections(x_game_key)
Get collections

Returns an array of active item collections for the game associated with the provided `X-Game-Key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**Vec<crate::models::GetCollections200ResponseInner>**](getCollections_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_collection_role

> crate::models::TransactionModel grant_collection_role(collection_id, x_authorization, x_password, grant_collection_role_request)
Grant collection role

Grants the provided role for the collection to the provided address or address associated with the provided walletId. Granted roles give different types of authority on behalf of the collection for specific players, addresses, or contracts to perform different types of permissioned collection operations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**grant_collection_role_request** | [**GrantCollectionRoleRequest**](GrantCollectionRoleRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_collection_item

> crate::models::TransactionModel mint_collection_item(collection_id, collection_item_id, x_authorization, x_password, mint_collection_item_request)
Mint collection item

Creates (mints) the specified quantity of the provided collectionItemId to the provided wallet address or wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**mint_collection_item_request** | [**MintCollectionItemRequest**](MintCollectionItemRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_collection_role

> crate::models::TransactionModel revoke_collection_role(collection_id, x_authorization, x_password, revoke_collection_role_request)
Revoke collection role

Revokes the provided role for the collection to the provided address or address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**revoke_collection_role_request** | [**RevokeCollectionRoleRequest**](RevokeCollectionRoleRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_collection_approval

> crate::models::TransactionModel set_collection_approval(collection_id, x_authorization, x_password, set_collection_approval_request)
Set collection approval

Sets approval for the provided address or wallet address associated with the provided walletId to operate on behalf of the authenticating game or player's owned items for this collection. Setting an approved value of `true` allows the provided address or address associated with the provided walletId to transfer and burn items from this collection on behalf of the authenticated game or player's wallet address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**set_collection_approval_request** | [**SetCollectionApprovalRequest**](SetCollectionApprovalRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_collection_item_timelock

> crate::models::TransactionModel set_collection_item_timelock(collection_id, collection_item_id, x_authorization, x_password, set_collection_item_timelock_request)
Set collection item timelock

Sets the item timelock for the provided collection itemId. The timelock is a unix timestamp (in seconds) that defines a period in time of when an item may be transferred by players. Until the timelock timestamp has passed, the itemId for the given timelock may not be transferred, sold, traded, etc. A timelock of 0 (default) means that there is no timelock set on the itemId and it can be freely transferred, traded, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_password** | **String** | The password of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**set_collection_item_timelock_request** | [**SetCollectionItemTimelockRequest**](SetCollectionItemTimelockRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_collection_item

> crate::models::TransactionModel transfer_collection_item(collection_id, collection_item_id, x_authorization, x_password, transfer_collection_item_request)
Transfer collection item

Transfers specified quantity of itemId to the provided wallet address or wallet address associated with the provided walletId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Any collection id within the MetaFab ecosystem. | [required] |
**collection_item_id** | **f32** | Any item id for the collection. Zero, or a positive integer. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_password** | **String** | The password of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**transfer_collection_item_request** | [**TransferCollectionItemRequest**](TransferCollectionItemRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

