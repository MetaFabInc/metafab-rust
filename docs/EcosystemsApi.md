# \EcosystemsApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_ecosystem_game**](EcosystemsApi.md#approve_ecosystem_game) | **POST** /v1/ecosystems/{ecosystemId}/games | Approve ecosystem game
[**auth_ecosystem**](EcosystemsApi.md#auth_ecosystem) | **GET** /v1/ecosystems/auth | Authenticate ecosystem
[**auth_profile**](EcosystemsApi.md#auth_profile) | **GET** /v1/profiles/auth | Authenticate profile
[**auth_profile_player**](EcosystemsApi.md#auth_profile_player) | **GET** /v1/profiles/{profileId}/games/{gameId}/players/auth | Authenticate profile player
[**create_ecosystem**](EcosystemsApi.md#create_ecosystem) | **POST** /v1/ecosystems | Create ecosystem
[**create_profile**](EcosystemsApi.md#create_profile) | **POST** /v1/profiles | Create profile
[**create_profile_player**](EcosystemsApi.md#create_profile_player) | **POST** /v1/profiles/{profileId}/games/{gameId}/players | Create profile player
[**get_ecosystem**](EcosystemsApi.md#get_ecosystem) | **GET** /v1/ecosystems/{ecosystemId} | Get ecosystem
[**get_ecosystem_game**](EcosystemsApi.md#get_ecosystem_game) | **GET** /v1/ecosystems/{ecosystemId}/games/{gameId} | Get ecosystem game
[**get_ecosystem_games**](EcosystemsApi.md#get_ecosystem_games) | **GET** /v1/ecosystems/{ecosystemId}/games | Get ecosystem games
[**get_profile_game**](EcosystemsApi.md#get_profile_game) | **GET** /v1/profiles/{profileId}/games/{gameId} | Get profile game
[**get_profile_games**](EcosystemsApi.md#get_profile_games) | **GET** /v1/profiles/{profileId}/games | Get profile games
[**unapprove_ecosystem_game**](EcosystemsApi.md#unapprove_ecosystem_game) | **DELETE** /v1/ecosystems/{ecosystemId}/games/{gameId} | Unapprove ecosystem game
[**update_ecosystem**](EcosystemsApi.md#update_ecosystem) | **PATCH** /v1/ecosystems/{ecosystemId} | Update ecosystem
[**update_profile**](EcosystemsApi.md#update_profile) | **PATCH** /v1/profiles/{profileId} | Update profile
[**update_profile_player**](EcosystemsApi.md#update_profile_player) | **PATCH** /v1/profiles/{profileId}/games/{gameId}/players/{playerId} | Update profile player



## approve_ecosystem_game

> approve_ecosystem_game(ecosystem_id, x_authorization, approve_ecosystem_game_request)
Approve ecosystem game

Approves a game for an ecosystem. By approving a game, it allows that game to integrate the ability for profile accounts from an ecosystem to login directly to the approved game and play. This also allows games to request access to assets held at the profile level for the game to frictionlessly interact with on behalf of the profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ecosystem_id** | **String** | The ecosystem id of the authenticating ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating ecosystem. | [required] |
**approve_ecosystem_game_request** | [**ApproveEcosystemGameRequest**](ApproveEcosystemGameRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_ecosystem

> crate::models::EcosystemModel auth_ecosystem()
Authenticate ecosystem

Returns an existing ecosystem object containing authorization keys when provided a valid email (in place of username) and password login using Basic Auth.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EcosystemModel**](EcosystemModel.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_profile

> crate::models::AuthProfile200Response auth_profile(x_ecosystem_key)
Authenticate profile

Returns an existing profile object containing access token, wallet, and other details when provided a valid profile username and password login using Basic Auth.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_ecosystem_key** | **String** | The `publishedKey` of a specific ecosystem. This can be shared or included in clients, websites, etc. | [required] |

### Return type

[**crate::models::AuthProfile200Response**](authProfile_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_profile_player

> crate::models::AuthPlayer200Response auth_profile_player(profile_id, game_id, x_authorization, x_wallet_decrypt_key, x_username)
Authenticate profile player

Returns an existing player object containing access token, wallet, wallet decrypt key, profile authorization and other details for a game when provided profile authentication and the player's username.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id of the authenticating profile. | [required] |
**game_id** | **String** | Any game id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating profile. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating profile. Required to decrypt and perform blockchain transactions with the profile wallet. | [required] |
**x_username** | **String** | The username of a player. | [required] |

### Return type

[**crate::models::AuthPlayer200Response**](authPlayer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ecosystem

> crate::models::EcosystemModel create_ecosystem(create_ecosystem_request)
Create ecosystem

Create a new ecosystem. An ecosystem is a parent entity that many profiles live under for a given ecosystem of games. Ecosystems allow your players to create one profile within your ecosystem that allows a single account and wallet to be used across all of the approved games in your ecosystem that they play.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ecosystem_request** | [**CreateEcosystemRequest**](CreateEcosystemRequest.md) |  | [required] |

### Return type

[**crate::models::EcosystemModel**](EcosystemModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_profile

> crate::models::AuthProfile200Response create_profile(x_ecosystem_key, create_profile_request)
Create profile

Create a new profile. Profiles are automatically associated with an internally managed wallet. Profiles can be thought of as a umbrella account that can be used to sign into and create player accounts across many games and have a singular asset store wallet at the profile level that can be used across all connected player accounts for games those player accounts are a part of.  Profiles are associated to a parent ecosystem of games. This allows an ecosystem to approve a permissioned set of games that can request authorized wallet permissions from profiles of players for their game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_ecosystem_key** | **String** | The `publishedKey` of a specific ecosystem. This can be shared or included in clients, websites, etc. | [required] |
**create_profile_request** | [**CreateProfileRequest**](CreateProfileRequest.md) |  | [required] |

### Return type

[**crate::models::AuthProfile200Response**](authProfile_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_profile_player

> crate::models::AuthPlayer200Response create_profile_player(profile_id, game_id, x_authorization, x_wallet_decrypt_key, create_profile_player_request)
Create profile player

Creates a new player account for the provided game id linked to the authenticating profile. The created player account will default to using the parent profile's wallet for any transactions, wallet content balance checks and verifications, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id of the authenticating profile. | [required] |
**game_id** | **String** | Any game id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating profile. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating profile. Required to decrypt and perform blockchain transactions with the profile wallet. | [required] |
**create_profile_player_request** | [**CreateProfilePlayerRequest**](CreateProfilePlayerRequest.md) |  | [required] |

### Return type

[**crate::models::AuthPlayer200Response**](authPlayer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ecosystem

> crate::models::PublicEcosystem get_ecosystem(ecosystem_id)
Get ecosystem

Returns a ecosystem object for the provided ecosystem id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ecosystem_id** | **String** | Any ecosystem id within the MetaFab platform. | [required] |

### Return type

[**crate::models::PublicEcosystem**](PublicEcosystem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ecosystem_game

> crate::models::PublicGame get_ecosystem_game(ecosystem_id, game_id)
Get ecosystem game

Returns a game object for the provided game id that the ecosystem has approved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ecosystem_id** | **String** | Any ecosystem id within the MetaFab platform. | [required] |
**game_id** | **String** | Any game id within the MetaFab platform. | [required] |

### Return type

[**crate::models::PublicGame**](PublicGame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ecosystem_games

> Vec<crate::models::PublicGame> get_ecosystem_games(ecosystem_id)
Get ecosystem games

Returns an array of games the ecosystem has approved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ecosystem_id** | **String** | Any ecosystem id within the MetaFab platform. | [required] |

### Return type

[**Vec<crate::models::PublicGame>**](PublicGame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_game

> crate::models::GetProfileGames200ResponseInner get_profile_game(profile_id, game_id, x_authorization)
Get profile game

Returns a game this profile has connected player accounts for.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id of the authenticating profile. | [required] |
**game_id** | **String** | Any game id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating profile. | [required] |

### Return type

[**crate::models::GetProfileGames200ResponseInner**](getProfileGames_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_games

> Vec<crate::models::GetProfileGames200ResponseInner> get_profile_games(profile_id, x_authorization)
Get profile games

Returns an array of games the authorized profile has connected player accounts for.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id of the authenticating profile. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating profile. | [required] |

### Return type

[**Vec<crate::models::GetProfileGames200ResponseInner>**](getProfileGames_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unapprove_ecosystem_game

> unapprove_ecosystem_game(ecosystem_id, game_id, x_authorization)
Unapprove ecosystem game

Unapproves a game for an ecosystem. The game will no longer be able to allow profiles from the ecosystem to login. All profile permissions approved for the game will also be revoked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ecosystem_id** | **String** | The ecosystem id of the authenticating ecosystem. | [required] |
**game_id** | **String** | Any game id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating ecosystem. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ecosystem

> crate::models::EcosystemModel update_ecosystem(ecosystem_id, x_authorization, update_ecosystem_request)
Update ecosystem

Update various fields specific to an ecosystem. Such as changing its password, resetting its published or secret key, or updating its approved games.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ecosystem_id** | **String** | The ecosystem id of the authenticating ecosystem. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating ecosystem. | [required] |
**update_ecosystem_request** | [**UpdateEcosystemRequest**](UpdateEcosystemRequest.md) |  | [required] |

### Return type

[**crate::models::EcosystemModel**](EcosystemModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile

> crate::models::ProfileModel update_profile(profile_id, x_authorization, update_profile_request)
Update profile

Update various fields specific to a profile. Such as changing its password and resetting its access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id of the authenticating profile. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating profile. | [required] |
**update_profile_request** | [**UpdateProfileRequest**](UpdateProfileRequest.md) |  | [required] |

### Return type

[**crate::models::ProfileModel**](ProfileModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile_player

> crate::models::UpdateProfilePlayer200Response update_profile_player(profile_id, game_id, player_id, x_authorization, x_wallet_decrypt_key, update_profile_player_request)
Update profile player

Update various fields specific to a player. Such as changing its permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id of the authenticating profile. | [required] |
**game_id** | **String** | Any game id within the MetaFab platform. | [required] |
**player_id** | **String** | Any player id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `accessToken` of the authenticating profile. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating profile. Required to decrypt and perform blockchain transactions with the profile wallet. | [required] |
**update_profile_player_request** | [**UpdateProfilePlayerRequest**](UpdateProfilePlayerRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateProfilePlayer200Response**](updateProfilePlayer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

