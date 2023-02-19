# \ContractsApi

All URIs are relative to *https://api.trymetafab.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_contract**](ContractsApi.md#create_contract) | **POST** /v1/contracts | Create custom contract
[**get_contracts**](ContractsApi.md#get_contracts) | **GET** /v1/contracts | Get contracts
[**read_contract**](ContractsApi.md#read_contract) | **GET** /v1/contracts/{contractId}/reads | Read contract data
[**transfer_contract_ownership**](ContractsApi.md#transfer_contract_ownership) | **POST** /v1/contracts/{contractId}/owners | Transfer contract ownership
[**upgrade_contract_trusted_forwarder**](ContractsApi.md#upgrade_contract_trusted_forwarder) | **POST** /v1/contracts/{contractId}/forwarders | Upgrade contract trusted forwarder
[**write_contract**](ContractsApi.md#write_contract) | **POST** /v1/contracts/{contractId}/writes | Write contract data



## create_contract

> crate::models::ContractModel create_contract(x_authorization, create_contract_request)
Create custom contract

Create a MetaFab custom contract entry from an existing contract address and contract abi. This allows the game and players belonging to the authenticated game to interact with the contract's read and write functions through MetaFab's read and write contract endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**create_contract_request** | [**CreateContractRequest**](CreateContractRequest.md) |  | [required] |

### Return type

[**crate::models::ContractModel**](ContractModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts

> Vec<crate::models::ContractModel> get_contracts(x_game_key)
Get contracts

Returns an array of active contracts deployed by the game associated with the provided `X-Game-Key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_game_key** | **String** | The `publishedKey` of a specific game. This can be shared or included in game clients, websites, etc. | [required] |

### Return type

[**Vec<crate::models::ContractModel>**](ContractModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_contract

> serde_json::Value read_contract(contract_id, func, args)
Read contract data

Oftentimes you'll want to query and retrieve some data from a contract. This is incredibly easy to do for any contract deployed through MetaFab.  Using this endpoint, you can get the data returned by any readable function listed in a contracts ABI. This could be things like querying the totalSupply of a currency contract, the number of owners of an items contract, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | Any contract id within the MetaFab platform. | [required] |
**func** | **String** | A contract function name. This can be any valid function from the the ABI of the contract you are interacting with. For example, `balanceOf`. | [required] |
**args** | Option<**String**> | A comma seperated list of basic data type arguments. This is optional and only necessary if the function being invoked requires arguments per the contract ABI. For example, `123,\"Hello\",false`. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_contract_ownership

> crate::models::TransactionModel transfer_contract_ownership(contract_id, x_authorization, x_wallet_decrypt_key, transfer_contract_ownership_request)
Transfer contract ownership

Transfer ownership and control of a MetaFab deployed smart contract to another wallet you control. Transferring control does not disrupt your usage of MetaFab APIs and can be done so without causing any service outages for your game. The new owner wallet will have full control over any relevant item collections and marketplace related pages this contract may be associated with, such as for MetaFab item or NFT contracts.  Your game's custodial wallet will retain a `MANAGER_ROLE` on your contracts, allowing you to still use MetaFab APIs without issue while you retain full contract ownership and the contract's administrator role. If ever you want eject from using the MetaFab APIs but still retain your deployed smart contracts, you can revoke the `MANAGER_ROLE` from your game's custodial wallet address for your contract. We do not lock you into our systems.  Please be certain that the wallet address you transfer ownership to is one you control. Once ownership and admin permissions are transferred, your game's custodial wallet no longer has permission to reassign ownership or administrative priveleges for your contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | Any contract id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**transfer_contract_ownership_request** | [**TransferContractOwnershipRequest**](TransferContractOwnershipRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_contract_trusted_forwarder

> crate::models::TransactionModel upgrade_contract_trusted_forwarder(contract_id, x_authorization, x_wallet_decrypt_key, upgrade_contract_trusted_forwarder_request)
Upgrade contract trusted forwarder

In rare circumstances, you may need to upgrade the underlying trusted forwarder contract address attached to your game's contracts. Using this endpoint, you can provide a new trusted forwarder contract address to assign to any of your contracts that implement the `upgradeTrustedForwarder` function.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | Any contract id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of the authenticating game. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game. Required to decrypt and perform blockchain transactions with the game primary wallet. | [required] |
**upgrade_contract_trusted_forwarder_request** | [**UpgradeContractTrustedForwarderRequest**](UpgradeContractTrustedForwarderRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_contract

> crate::models::TransactionModel write_contract(contract_id, x_authorization, x_wallet_decrypt_key, write_contract_request)
Write contract data

MetaFab's convenience endpoints for contract interactions may not be flexible enough depending on your use case. For these situations, you can interact with contracts and create transactions directly.  Using this endpoint, you can execute a transaction for any writeable contract method as defined in the contract's ABI for the MetaFab contractId provided. Both Games and Player resources have authority to use this endpoint to execute transactions against any valid MetaFab contractId.  Additionally, MetaFab will automatically attempt to perform a gasless transaction for players interacting with a contract through this endpoint. Gasless transactions by players through this endpoint will only work if the target contract was deployed through MetaFab or supports MetaFab's ERC2771 trusted forwarder contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | Any contract id within the MetaFab platform. | [required] |
**x_authorization** | **String** | The `secretKey` of a specific game or the `accessToken` of a specific player. | [required] |
**x_wallet_decrypt_key** | **String** | The `walletDecryptKey` of the authenticating game or player. Required to decrypt and perform blockchain transactions with the game or player primary wallet. | [required] |
**write_contract_request** | [**WriteContractRequest**](WriteContractRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionModel**](TransactionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

