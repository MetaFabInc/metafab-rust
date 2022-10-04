# UpdateGameRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A new name. Replaces the game's current name. | [optional]
**email** | Option<**String**> | A new email address. The game's old email will no longer be valid for account authentication. `currentPassword` must also be provided. | [optional]
**current_password** | Option<**String**> | The game's current password. Must be provided if setting `newPassword` or `email`. | [optional]
**new_password** | Option<**String**> | A new password. The game's old password will no longer be valid. | [optional]
**rpcs** | Option<**::std::collections::HashMap<String, String>**> | Sets a custom RPC for your game to use instead of MetaFab's default RPCs for the chain(s) you specify.  Expects a JSON object containing key value pairs of supported `chain` -> `rpc url`. Only the chain names provided as keys in the object will be explicitly overriden. To delete a custom RPC for your game, provide the chain name to delete as a key in the provided object and `null` as the value.  Set RPC example, `{ MATIC: 'https://polygon-rpc.com' }` Delete RPC example, `{ MATIC: null }` | [optional]
**reset_published_key** | Option<**bool**> | Revokes the game's previous published key and returns a new one if true. | [optional]
**reset_secret_key** | Option<**bool**> | Revokes the game's previous secret key and returns a new on if true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


