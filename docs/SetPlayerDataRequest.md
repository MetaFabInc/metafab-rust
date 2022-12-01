# SetPlayerDataRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protected_data** | Option<[**serde_json::Value**](.md)> | protectedData can only be set if `X-Authorization` includes credentials for the game the target player is a part of. Expects an arbitrary object allowed to contain any set of properties and nested data within those properties, including arrays.  protectedData is great for storing sensitive player data like tracking experience points, off-chain inventories, save states, and more - things that players shouldn't have the ability to directly change themselves. | [optional]
**public_data** | Option<[**serde_json::Value**](.md)> | publicData can be set if `X-Authorization` includes credentials for the target player or game the player is a part of. Expects an arbitrary object allowed to contain any set of properties and nested data within those properties, including arrays.  publicData is great for storing player preferences like in-game settings, non-sensitive data and more. Anything that a player should have the ability to directly change themselves without client or server verification can be stored in publicData. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


