# \BlockApi

All URIs are relative to *http://localhost:3000/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_block_extended**](BlockApi.md#rest_block_extended) | **Get** /block/{blockHash} | Get block by hash.
[**rest_block_notxdetails**](BlockApi.md#rest_block_notxdetails) | **Get** /block/notxdetails/{blockHash}.{format} | Get block by hash.


# **rest_block_extended**
> ::models::Block rest_block_extended(block_hash)
Get block by hash.

Given a block hash: returns a block, in binary, hex-encoded binary or JSON formats. The HTTP request and response are both handled entirely in-memory, thus making maximum memory usage at least 2.66MB (1 MB max block, plus hex encoding) per request. With the /notxdetails/ option JSON response will only contain the transaction hash instead of the complete transaction details. The option only affects the JSON response.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **block_hash** | **String**| the block hash | 

### Return type

[**::models::Block**](Block.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/octet-stream, text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rest_block_notxdetails**
> ::models::Block rest_block_notxdetails(block_hash, format)
Get block by hash.

Given a block hash: returns a block, in binary, hex-encoded binary or JSON formats. The HTTP request and response are both handled entirely in-memory, thus making maximum memory usage at least 2.66MB (1 MB max block, plus hex encoding) per request. With the /notxdetails/ option JSON response will only contain the transaction hash instead of the complete transaction details. The option only affects the JSON response.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **block_hash** | **String**| The block hash | 
  **format** | **String**| The expected format | 

### Return type

[**::models::Block**](Block.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/octet-stream, text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

