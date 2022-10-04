# UpdatePlayerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_password** | Option<**String**> | The player's current password. Must be provided if setting `newPassword`. | [optional]
**new_password** | Option<**String**> | A new password. The player's old password will no longer be valid. | [optional]
**reset_access_token** | Option<**bool**> | Revokes the player's previous access token and returns a new one if true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


