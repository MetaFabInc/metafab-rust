# UpdateEcosystemRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A new name. Replaces the ecosystem's current name. | [optional]
**email** | Option<**String**> | A new email address. The ecosystem's old email will no longer be valid for account authentication. `currentPassword` must also be provided. | [optional]
**current_password** | Option<**String**> | The ecosystem's current password. Must be provided if setting `newPassword` or `email`. | [optional]
**new_password** | Option<**String**> | A new password. The ecosystem's old password will no longer be valid. | [optional]
**icon_image_base64** | Option<**String**> | A base64 string of the icon image for this ecosystem. Supported image formats are `jpg`, `jpeg`, `png`, `gif` Recommended size is 512x512 pixels, or 1:1 aspect ratio. This image is used for your profile authorization flow and other MetaFab features for your ecosystem. | [optional]
**cover_image_base64** | Option<**String**> | A base64 string of the cover image for this ecosystem. Supported image formats are `jpg`, `jpeg`, `png`, `gif`. Recommended size is 1600x1000 pixels, or 16:10 aspect ratio.  This image is used as the background image for your profile authorization flow and other MetaFab features for your ecosystem. | [optional]
**primary_color_hex** | Option<**String**> | A valid hex color code. This color is used for your profile authorization flow to control the color of buttons and other brandable MetaFab features for your ecosystem. | [optional]
**reset_published_key** | Option<**bool**> | Revokes the ecosystem's previous published key and returns a new one if true. | [optional]
**reset_secret_key** | Option<**bool**> | Revokes the ecosystem's previous secret key and returns a new on if true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


