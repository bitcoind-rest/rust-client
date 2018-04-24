# \MemoryPoolApi

All URIs are relative to *http://localhost:3000/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_getutxos**](MemoryPoolApi.md#rest_getutxos) | **Get** /getutxos/checkmempool/{txHash}-{txOutput}.{format} | Returns Unspent Transaction (TX) Outputs
[**rest_headers**](MemoryPoolApi.md#rest_headers) | **Get** /headers/{headerCount}/{blockHash}.{format} | Returns headers.
[**rest_mempool_contents**](MemoryPoolApi.md#rest_mempool_contents) | **Get** /mempool/contents.json | Returns transactions in the TX mempool.
[**rest_mempool_info**](MemoryPoolApi.md#rest_mempool_info) | **Get** /mempool/info.json | Returns various information about the TX mempool.


# **rest_getutxos**
> ::models::InlineResponse200 rest_getutxos(tx_hash, tx_output, format)
Returns Unspent Transaction (TX) Outputs

Only supports JSON as output format.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **tx_hash** | **String**| The transaction hash | 
  **tx_output** | **String**| The transaction output | 
  **format** | **String**| The expected format | 

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rest_headers**
> String rest_headers(header_count, block_hash, format)
Returns headers.

Only supports BIN and HEX as output format.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **header_count** | **i32**| The header count | 
  **block_hash** | **String**| The block hash | 
  **format** | **String**| The expected format | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rest_mempool_contents**
> ::models::InlineResponseDefault rest_mempool_contents()
Returns transactions in the TX mempool.

Only supports JSON as output format.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponseDefault**](inline_response_default.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **rest_mempool_info**
> ::models::MemoryPool rest_mempool_info()
Returns various information about the TX mempool.

Only supports JSON as output format.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::MemoryPool**](MemoryPool.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

