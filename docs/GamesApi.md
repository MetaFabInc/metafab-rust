# \GamesApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_game**](GamesApi.md#auth_game) | **GET** /v1/games/auth | Authenticate game
[**create_game**](GamesApi.md#create_game) | **POST** /v1/games | Create game
[**get_game**](GamesApi.md#get_game) | **GET** /v1/games/{gameId} | Get game
[**update_game**](GamesApi.md#update_game) | **PATCH** /v1/games/{gameId} | Update game



## auth_game

> crate::models::AuthGame200Response auth_game()
Authenticate game

Returns an existing game object containing authorization keys and credentials when provided a valid email (in place of username) and password login using Basic Auth.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthGame200Response**](authGame_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game

> crate::models::AuthGame200Response create_game(create_game_request)
Create game

Create a new game. A game is the root entity required for all API interactions. Contracts, currencies, items and more are deployed by games, player accounts are created and registered to games, etc.  To use any of MetaFab's services, you must first create a game through this endpoint.  After creating your game through this endpoint, a verification email will be sent to the email address used. Before you can access any of MetaFab's features, you'll need to click the link contained in the verification email to verify your account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_game_request** | [**CreateGameRequest**](CreateGameRequest.md) |  | [required] |

### Return type

[**crate::models::AuthGame200Response**](authGame_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game

> crate::models::PublicGame get_game(game_id)
Get game

Returns a game object for the provided game id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | Any game id within the MetaFab ecosystem. | [required] |

### Return type

[**crate::models::PublicGame**](PublicGame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_game

> crate::models::GameModel update_game(game_id, x_authorization, update_game_request)
Update game

Update various fields specific to a game. Such as changing its password, resetting its published or secret key, or updating its RPCs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | Any game id within the MetaFab ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**update_game_request** | [**UpdateGameRequest**](UpdateGameRequest.md) |  | [required] |

### Return type

[**crate::models::GameModel**](GameModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

