# \PlayersApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_player**](PlayersApi.md#auth_player) | **GET** /v1/players/auth | Authenticate player
[**create_player**](PlayersApi.md#create_player) | **POST** /v1/players | Create player
[**get_player**](PlayersApi.md#get_player) | **GET** /v1/players/{playerId} | Get player
[**get_player_data**](PlayersApi.md#get_player_data) | **GET** /v1/players/{playerId}/data | Get player data
[**get_players**](PlayersApi.md#get_players) | **GET** /v1/players | Get players
[**remove_player_connected_wallet**](PlayersApi.md#remove_player_connected_wallet) | **DELETE** /v1/players/{playerId}/wallets/{playerWalletId} | Remove player connected wallet
[**set_player_connected_wallet**](PlayersApi.md#set_player_connected_wallet) | **POST** /v1/players/{playerId}/wallets | Set player connected wallet
[**set_player_data**](PlayersApi.md#set_player_data) | **POST** /v1/players/{playerId}/data | Set player data
[**update_player**](PlayersApi.md#update_player) | **PATCH** /v1/players/{playerId} | Update player



## auth_player

> crate::models::AuthPlayer200Response auth_player(x_game_key)
Authenticate player

Returns an existing player object containing access token, wallet, and other details for a game when provided a valid username and password login using Basic Auth.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**crate::models::AuthPlayer200Response**](authPlayer_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_player

> crate::models::AuthPlayer200Response create_player(x_game_key, create_player_request)
Create player

Create a new player for a game. Players are automatically associated with an internally managed wallet.  Player access tokens can be used to directly interact with any MetaFab managed contracts, currencies, items collections, marketplaces and more. Player interactions are also gasless by default, completely removing all crypto friction for players to engage with your MetaFab supported games.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |
**create_player_request** | [**CreatePlayerRequest**](CreatePlayerRequest.md) |  | [required] |

### Return type

[**crate::models::AuthPlayer200Response**](authPlayer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player

> crate::models::PublicPlayer get_player(player_id)
Get player

Returns a player object for the provided player id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Any player id within the MetaFab platform. | [required] |

### Return type

[**crate::models::PublicPlayer**](PublicPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player_data

> crate::models::GetPlayerData200Response get_player_data(player_id)
Get player data

Returns the latest public and protected data as an object for the provided playerId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Any player id within the MetaFab platform. | [required] |

### Return type

[**crate::models::GetPlayerData200Response**](getPlayerData_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_players

> Vec<crate::models::PublicPlayer> get_players(x_authorization)
Get players

Returns all players for the authenticated game as an array of player objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |

### Return type

[**Vec<crate::models::PublicPlayer>**](PublicPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_player_connected_wallet

> remove_player_connected_wallet(player_id, player_wallet_id, remove_player_connected_wallet_request)
Remove player connected wallet

Removes an external wallet from a player account. The player's wallet is reverted to their custodial wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Any player id within the MetaFab platform. | [required] |
**player_wallet_id** | **String** | Any player wallet id within the MetaFab platform. | [required] |
**remove_player_connected_wallet_request** | [**RemovePlayerConnectedWalletRequest**](RemovePlayerConnectedWalletRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_player_connected_wallet

> crate::models::SetPlayerConnectedWallet200Response set_player_connected_wallet(player_id, x_authorization, set_player_connected_wallet_request)
Set player connected wallet

Sets an external wallet as the wallet for a player account. The set wallet can transact gaslessly with all MetaFab related systems through the same MetaFab API calls as custodial wallets without requiring transaction signing or private keys.  This is done through an internal system MetaFab has created that allows an external connected wallet to delegate transaction signing for a specific game's set of contracts to a player's password protected custodial wallet. This allow the custodial wallet to sign and submit transactions to a specific game's related contracts as if they were signed and submitted by the connected external wallet. This also means that all earned tokens, purchased items and any interactions happen and are recorded on chain as the external connected wallet. No additional logic needs to be writted by developers to support both custodial and external wallets, everything just works.  Finally, this endpoint is meant for advanced users. The majority of developers who want to implement external wallet support for their game can do so without any extra work through our whitelabeled wallet connection feature that implements this endpoint underneath the hood without any required work.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | The player id of the authenticating player. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating player. | [required] |
**set_player_connected_wallet_request** | [**SetPlayerConnectedWalletRequest**](SetPlayerConnectedWalletRequest.md) |  | [required] |

### Return type

[**crate::models::SetPlayerConnectedWallet200Response**](setPlayerConnectedWallet_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_player_data

> crate::models::GetPlayerData200Response set_player_data(player_id, x_authorization, set_player_data_request)
Set player data

Creates or updates public and/or protected data for the provided playerId. Data updates are performed using deep merging. This means that when you update any top level or nested properties specific to player public or protected data, you only need to include the properties you are making changes to. Any existing properties not included in request body arguments will be retained on the player data object.  Please note, When writing an array type for a player, arrays do not follow the deep merge approach. If you add or remove an item from an array, the entire array must be passed as an argument when updating the related property for player public or protected data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Any player id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**set_player_data_request** | [**SetPlayerDataRequest**](SetPlayerDataRequest.md) |  | [required] |

### Return type

[**crate::models::GetPlayerData200Response**](getPlayerData_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_player

> crate::models::UpdatePlayer200Response update_player(player_id, x_authorization, update_player_request)
Update player

Update various fields specific to a player. Such as changing its password and resetting its access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | The player id of the authenticating player. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating player. | [required] |
**update_player_request** | [**UpdatePlayerRequest**](UpdatePlayerRequest.md) |  | [required] |

### Return type

[**crate::models::UpdatePlayer200Response**](updatePlayer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

