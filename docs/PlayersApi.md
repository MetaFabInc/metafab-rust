# \PlayersApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_player**](PlayersApi.md#auth_player) | **GET** /v1/players | Authenticate player
[**create_player**](PlayersApi.md#create_player) | **POST** /v1/players | Create player
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


## update_player

> crate::models::PlayerModel update_player(player_id, x_authorization, update_player_request)
Update player

Update various fields specific to a player. Such as changing its password and resetting its access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Any player id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating player. | [required] |
**update_player_request** | [**UpdatePlayerRequest**](UpdatePlayerRequest.md) |  | [required] |

### Return type

[**crate::models::PlayerModel**](PlayerModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

