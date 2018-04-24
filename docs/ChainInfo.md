# ChainInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chain** | **String** | Current network name as defined in BIP70 (main, test, regtest) | [optional] [default to null]
**blocks** | **i64** | The current number of blocks processed in the server | [optional] [default to null]
**headers** | **i64** | The current number of headers we have validated | [optional] [default to null]
**bestblockhash** | **String** | The hash of the currently best block | [optional] [default to null]
**difficulty** | **i64** | The current difficulty | [optional] [default to null]
**mediantime** | **i32** | The median time of the 11 blocks before the most recent block on the blockchain | [optional] [default to null]
**verificationprogress** | **i32** | Estimate of verification progress [0..1] | [optional] [default to null]
**initialblockdownload** | **bool** |  | [optional] [default to null]
**chainwork** | **String** | Total amount of work in active chain, in hexadecimal | [optional] [default to null]
**size_on_disk** | **i64** |  | [optional] [default to null]
**pruned** | **bool** | If the blocks are subject to pruning | [optional] [default to null]
**softforks** | [**Vec<::models::Bip>**](BIP.md) | status of softforks in progress | [optional] [default to null]
**bip9_softforks** | [***::models::ChainInfoBip9Softforks**](ChainInfo_bip9_softforks.md) |  | [optional] [default to null]
**warnings** | **String** | An eventual warning on the current build status. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


