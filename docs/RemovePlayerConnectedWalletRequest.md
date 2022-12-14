# RemovePlayerConnectedWalletRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | The address of the external wallet to remove from a player. | 
**nonce** | **i32** | Any positive integer that hasn't been used previously to connect or disconnect this external wallet. Must be the same nonce used to generate the `signature`. | 
**signature** | **String** | A signature string generated by signing a keccak256 hash generated with abiCoder encode arguments of `[ 'bytes32', 'address', 'bool', 'address', 'uint256' ]` and `[ keccak256(gameId), delegate wallet unapproved (player's custodial wallet address), false, external wallet address, any unused nonce number ]`. | 
**chain** | **String** | The blockchain you want to remove this wallet connection on. If you need to remove it on multiple blockchains, make multiple requests using different `chain` arguments. Support for new blockchains are added over time. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


