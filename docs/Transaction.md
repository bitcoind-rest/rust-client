# Transaction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f32** | The transaction amount in BTC | [optional] [default to null]
**fee** | **f32** | The amount of the fee in BTC. This is negative and only available for the send category of transactions. | [optional] [default to null]
**confirmations** | **i64** | The number of confirmations | [optional] [default to null]
**blockhash** | **String** | The block hash | [optional] [default to null]
**blockindex** | **i64** | The index of the transaction in the block that includes it | [optional] [default to null]
**blocktime** | **i32** | The time in seconds since epoch (1 Jan 1970 GMT) | [optional] [default to null]
**txid** | **String** | The transaction id | [optional] [default to null]
**txhash** | **String** | The transaction hash | [optional] [default to null]
**version** | **i32** |  | [optional] [default to null]
**size** | **i32** |  | [optional] [default to null]
**vsize** | **i32** |  | [optional] [default to null]
**locktime** | **i32** |  | [optional] [default to null]
**time** | **i32** | The transaction time in seconds since epoch (1 Jan 1970 GMT) | [optional] [default to null]
**timereceived** | **i32** | The time received in seconds since epoch (1 Jan 1970 GMT) | [optional] [default to null]
**bip125_replaceable** | **String** | Whether this transaction could be replaced due to BIP125 (replace-by-fee); may be unknown for unconfirmed transactions not in the mempool | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


