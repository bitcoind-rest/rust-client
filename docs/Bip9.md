# Bip9

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** |  | [optional] [default to null]
**start_time** | **i64** | The starttime specifies a minimum median time past of a block at which the bit gains its meaning. | [optional] [default to null]
**timeout** | **i64** | The timeout specifies a time at which the deployment is considered failed. If the median time past of a block &gt;&#x3D; timeout and the soft fork has not yet locked in (including this block&#39;s bit state), the deployment is considered failed on all descendants of the block. | [optional] [default to null]
**since** | **i64** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


