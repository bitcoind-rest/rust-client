# Rust API client for swagger

The REST API can be enabled with the `-rest` option. The interface runs on the same port as the JSON-RPC interface, by default port `8332` for **mainnet**, port `18332` for **testnet**, and port `18443` for **regtest**.

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 0.16
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:3000/rest*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BlockApi* | [**rest_block_extended**](docs/BlockApi.md#rest_block_extended) | **Get** /block/{blockHash} | Get block by hash.
*BlockApi* | [**rest_block_notxdetails**](docs/BlockApi.md#rest_block_notxdetails) | **Get** /block/notxdetails/{blockHash}.{format} | Get block by hash.
*ChainApi* | [**rest_chaininfo**](docs/ChainApi.md#rest_chaininfo) | **Get** /chaininfo.json | Returns various state info regarding block chain processing.
*MemoryPoolApi* | [**rest_getutxos**](docs/MemoryPoolApi.md#rest_getutxos) | **Get** /getutxos/checkmempool/{txHash}-{txOutput}.{format} | Returns Unspent Transaction (TX) Outputs
*MemoryPoolApi* | [**rest_headers**](docs/MemoryPoolApi.md#rest_headers) | **Get** /headers/{headerCount}/{blockHash}.{format} | Returns headers.
*MemoryPoolApi* | [**rest_mempool_contents**](docs/MemoryPoolApi.md#rest_mempool_contents) | **Get** /mempool/contents.json | Returns transactions in the TX mempool.
*MemoryPoolApi* | [**rest_mempool_info**](docs/MemoryPoolApi.md#rest_mempool_info) | **Get** /mempool/info.json | Returns various information about the TX mempool.
*TransactionApi* | [**rest_tx**](docs/TransactionApi.md#rest_tx) | **Get** /tx/{txHash} | Get transaction by hash.


## Documentation For Models

 - [Bip](docs/Bip.md)
 - [Bip9](docs/Bip9.md)
 - [BipReject](docs/BipReject.md)
 - [Block](docs/Block.md)
 - [ChainInfo](docs/ChainInfo.md)
 - [ChainInfoBip9Softforks](docs/ChainInfoBip9Softforks.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponseDefault](docs/InlineResponseDefault.md)
 - [MemoryPool](docs/MemoryPool.md)
 - [ScriptPubKey](docs/ScriptPubKey.md)
 - [Transaction](docs/Transaction.md)
 - [UTxO](docs/UTxO.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author

johan@lepetitbloc.net

