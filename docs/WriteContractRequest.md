# WriteContractRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**func** | **String** | A contract function name. This can be any valid function from the the ABI of the contract you are interacting with. For example, `mint`. | 
**args** | Option<[**Vec<crate::models::WriteContractRequestArgsInner>**](writeContract_request_args_inner.md)> | An array of args. This is optional and only necessary if the function being invoked requires arguments per the contract ABI. For example, `[123, \"Hello\", false]`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


