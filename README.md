# Rust API client for metafab

 Complete MetaFab API references and guides can be found at: https://trymetafab.com

For more information, please visit [https://trymetafab.com](https://trymetafab.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.3.0
- Package version: 1.3.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `metafab` and add the following to `Cargo.toml` under `[dependencies]`:

```
metafab = { path = "./metafab" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.trymetafab.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ContractsApi* | [**create_contract**](docs/ContractsApi.md#create_contract) | **POST** /v1/contracts | Create custom contract
*ContractsApi* | [**get_contracts**](docs/ContractsApi.md#get_contracts) | **GET** /v1/contracts | Get contracts
*ContractsApi* | [**read_contract**](docs/ContractsApi.md#read_contract) | **GET** /v1/contracts/{contractId}/reads | Read contract data
*ContractsApi* | [**write_contract**](docs/ContractsApi.md#write_contract) | **POST** /v1/contracts/{contractId}/writes | Write contract data
*CurrenciesApi* | [**batch_transfer_currency**](docs/CurrenciesApi.md#batch_transfer_currency) | **POST** /v1/currencies/{currencyId}/batchTransfers | Batch transfer currency
*CurrenciesApi* | [**burn_currency**](docs/CurrenciesApi.md#burn_currency) | **POST** /v1/currencies/{currencyId}/burns | Burn currency
*CurrenciesApi* | [**create_currency**](docs/CurrenciesApi.md#create_currency) | **POST** /v1/currencies | Create currency
*CurrenciesApi* | [**get_currencies**](docs/CurrenciesApi.md#get_currencies) | **GET** /v1/currencies | Get currencies
*CurrenciesApi* | [**get_currency_balance**](docs/CurrenciesApi.md#get_currency_balance) | **GET** /v1/currencies/{currencyId}/balances | Get currency balance
*CurrenciesApi* | [**get_currency_fees**](docs/CurrenciesApi.md#get_currency_fees) | **GET** /v1/currencies/{currencyId}/fees | Get currency fees
*CurrenciesApi* | [**get_currency_role**](docs/CurrenciesApi.md#get_currency_role) | **GET** /v1/currencies/{currencyId}/roles | Get currency role
*CurrenciesApi* | [**grant_currency_role**](docs/CurrenciesApi.md#grant_currency_role) | **POST** /v1/currencies/{currencyId}/roles | Grant currency role
*CurrenciesApi* | [**mint_currency**](docs/CurrenciesApi.md#mint_currency) | **POST** /v1/currencies/{currencyId}/mints | Mint currency
*CurrenciesApi* | [**revoke_currency_role**](docs/CurrenciesApi.md#revoke_currency_role) | **DELETE** /v1/currencies/{currencyId}/roles | Revoke currency role
*CurrenciesApi* | [**set_currency_fees**](docs/CurrenciesApi.md#set_currency_fees) | **POST** /v1/currencies/{currencyId}/fees | Set currency fees
*CurrenciesApi* | [**transfer_currency**](docs/CurrenciesApi.md#transfer_currency) | **POST** /v1/currencies/{currencyId}/transfers | Transfer currency
*GamesApi* | [**auth_game**](docs/GamesApi.md#auth_game) | **GET** /v1/games/auth | Authenticate game
*GamesApi* | [**create_game**](docs/GamesApi.md#create_game) | **POST** /v1/games | Create game
*GamesApi* | [**get_game**](docs/GamesApi.md#get_game) | **GET** /v1/games/{gameId} | Get game
*GamesApi* | [**update_game**](docs/GamesApi.md#update_game) | **PATCH** /v1/games/{gameId} | Update game
*ItemsApi* | [**batch_mint_collection_items**](docs/ItemsApi.md#batch_mint_collection_items) | **POST** /v1/collections/{collectionId}/batchMints | Batch mint collection items
*ItemsApi* | [**batch_transfer_collection_items**](docs/ItemsApi.md#batch_transfer_collection_items) | **POST** /v1/collections/{collectionId}/batchTransfers | Batch transfer collection items
*ItemsApi* | [**burn_collection_item**](docs/ItemsApi.md#burn_collection_item) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/burns | Burn collection item
*ItemsApi* | [**create_collection**](docs/ItemsApi.md#create_collection) | **POST** /v1/collections | Create collection
*ItemsApi* | [**create_collection_item**](docs/ItemsApi.md#create_collection_item) | **POST** /v1/collections/{collectionId}/items | Create collection item
*ItemsApi* | [**get_collection_approval**](docs/ItemsApi.md#get_collection_approval) | **GET** /v1/collections/{collectionId}/approvals | Get collection approval
*ItemsApi* | [**get_collection_item**](docs/ItemsApi.md#get_collection_item) | **GET** /v1/collections/{collectionId}/items/{collectionItemId} | Get collection item
*ItemsApi* | [**get_collection_item_balance**](docs/ItemsApi.md#get_collection_item_balance) | **GET** /v1/collections/{collectionId}/items/{collectionItemId}/balances | Get collection item balance
*ItemsApi* | [**get_collection_item_balances**](docs/ItemsApi.md#get_collection_item_balances) | **GET** /v1/collections/{collectionId}/balances | Get collection item balances
*ItemsApi* | [**get_collection_item_supplies**](docs/ItemsApi.md#get_collection_item_supplies) | **GET** /v1/collections/{collectionId}/supplies | Get collection item supplies
*ItemsApi* | [**get_collection_item_supply**](docs/ItemsApi.md#get_collection_item_supply) | **GET** /v1/collections/{collectionId}/items/{collectionItemId}/supplies | Get collection item supply
*ItemsApi* | [**get_collection_item_timelock**](docs/ItemsApi.md#get_collection_item_timelock) | **GET** /v1/collections/{collectionId}/items/{collectionItemId}/timelocks | Get collection item timelock
*ItemsApi* | [**get_collection_items**](docs/ItemsApi.md#get_collection_items) | **GET** /v1/collections/{collectionId}/items | Get collection items
*ItemsApi* | [**get_collection_role**](docs/ItemsApi.md#get_collection_role) | **GET** /v1/collections/{collectionId}/roles | Get collection role
*ItemsApi* | [**get_collections**](docs/ItemsApi.md#get_collections) | **GET** /v1/collections | Get collections
*ItemsApi* | [**grant_collection_role**](docs/ItemsApi.md#grant_collection_role) | **POST** /v1/collections/{collectionId}/roles | Grant collection role
*ItemsApi* | [**mint_collection_item**](docs/ItemsApi.md#mint_collection_item) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/mints | Mint collection item
*ItemsApi* | [**revoke_collection_role**](docs/ItemsApi.md#revoke_collection_role) | **DELETE** /v1/collections/{collectionId}/roles | Revoke collection role
*ItemsApi* | [**set_collection_approval**](docs/ItemsApi.md#set_collection_approval) | **POST** /v1/collections/{collectionId}/approvals | Set collection approval
*ItemsApi* | [**set_collection_item_timelock**](docs/ItemsApi.md#set_collection_item_timelock) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/timelocks | Set collection item timelock
*ItemsApi* | [**transfer_collection_item**](docs/ItemsApi.md#transfer_collection_item) | **POST** /v1/collections/{collectionId}/items/{collectionItemId}/transfers | Transfer collection item
*LootboxesApi* | [**create_lootbox_manager**](docs/LootboxesApi.md#create_lootbox_manager) | **POST** /v1/lootboxManagers | Create lootbox manager
*LootboxesApi* | [**get_lootbox_manager_lootbox**](docs/LootboxesApi.md#get_lootbox_manager_lootbox) | **GET** /v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId} | Get lootbox manager lootbox
*LootboxesApi* | [**get_lootbox_manager_lootboxes**](docs/LootboxesApi.md#get_lootbox_manager_lootboxes) | **GET** /v1/lootboxManagers/{lootboxManagerId}/lootboxes | Get lootbox manager lootboxes
*LootboxesApi* | [**get_lootbox_managers**](docs/LootboxesApi.md#get_lootbox_managers) | **GET** /v1/lootboxManagers | Get lootbox managers
*LootboxesApi* | [**open_lootbox_manager_lootbox**](docs/LootboxesApi.md#open_lootbox_manager_lootbox) | **POST** /v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId}/opens | Open lootbox manager lootbox
*LootboxesApi* | [**remove_lootbox_manager_lootbox**](docs/LootboxesApi.md#remove_lootbox_manager_lootbox) | **DELETE** /v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId} | Remove lootbox manager lootbox
*LootboxesApi* | [**set_lootbox_manager_lootbox**](docs/LootboxesApi.md#set_lootbox_manager_lootbox) | **POST** /v1/lootboxManagers/{lootboxManagerId}/lootboxes | Set lootbox manager lootbox
*PlayersApi* | [**auth_player**](docs/PlayersApi.md#auth_player) | **GET** /v1/players/auth | Authenticate player
*PlayersApi* | [**create_player**](docs/PlayersApi.md#create_player) | **POST** /v1/players | Create player
*PlayersApi* | [**get_player**](docs/PlayersApi.md#get_player) | **GET** /v1/players/{playerId} | Get player
*PlayersApi* | [**get_player_data**](docs/PlayersApi.md#get_player_data) | **GET** /v1/players/{playerId}/data | Get player data
*PlayersApi* | [**get_players**](docs/PlayersApi.md#get_players) | **GET** /v1/players | Get players
*PlayersApi* | [**set_player_data**](docs/PlayersApi.md#set_player_data) | **POST** /v1/players/{playerId}/data | Set player data
*PlayersApi* | [**update_player**](docs/PlayersApi.md#update_player) | **PATCH** /v1/players/{playerId} | Update player
*ShopsApi* | [**create_shop**](docs/ShopsApi.md#create_shop) | **POST** /v1/shops | Create shop
*ShopsApi* | [**get_shop_offer**](docs/ShopsApi.md#get_shop_offer) | **GET** /v1/shops/{shopId}/items/{shopOfferId} | Get shop offer
*ShopsApi* | [**get_shop_offers**](docs/ShopsApi.md#get_shop_offers) | **GET** /v1/shops/{shopId}/offers | Get shop offers
*ShopsApi* | [**get_shops**](docs/ShopsApi.md#get_shops) | **GET** /v1/shops | Get shops
*ShopsApi* | [**remove_shop_offer**](docs/ShopsApi.md#remove_shop_offer) | **DELETE** /v1/shops/{shopId}/offers/{shopOfferId} | Remove shop offer
*ShopsApi* | [**set_shop_offer**](docs/ShopsApi.md#set_shop_offer) | **POST** /v1/shops/{shopId}/offers | Set shop offer
*ShopsApi* | [**use_shop_offer**](docs/ShopsApi.md#use_shop_offer) | **POST** /v1/shops/{shopId}/offers/{shopOfferId}/uses | Use shop offer
*ShopsApi* | [**withdraw_from_shop**](docs/ShopsApi.md#withdraw_from_shop) | **POST** /v1/shops/{shopId}/withdrawals | Withdraw from shop
*TransactionsApi* | [**get_transaction**](docs/TransactionsApi.md#get_transaction) | **GET** /v1/transactions/{transactionId} | Get transaction
*WalletsApi* | [**get_wallet_balances**](docs/WalletsApi.md#get_wallet_balances) | **GET** /v1/wallets/{walletId}/balances | Get wallet balances
*WalletsApi* | [**get_wallet_transactions**](docs/WalletsApi.md#get_wallet_transactions) | **GET** /v1/wallets/{walletId}/transactions | Get wallet transactions


## Documentation For Models

 - [AuthGame200Response](docs/AuthGame200Response.md)
 - [AuthGame200ResponseAllOf](docs/AuthGame200ResponseAllOf.md)
 - [AuthGame200ResponseAllOf1](docs/AuthGame200ResponseAllOf1.md)
 - [AuthPlayer200Response](docs/AuthPlayer200Response.md)
 - [BatchMintCollectionItemsRequest](docs/BatchMintCollectionItemsRequest.md)
 - [BatchTransferCollectionItemsRequest](docs/BatchTransferCollectionItemsRequest.md)
 - [BatchTransferCurrencyRequest](docs/BatchTransferCurrencyRequest.md)
 - [BurnCollectionItemRequest](docs/BurnCollectionItemRequest.md)
 - [BurnCurrencyRequest](docs/BurnCurrencyRequest.md)
 - [CollectionModel](docs/CollectionModel.md)
 - [ContractModel](docs/ContractModel.md)
 - [CreateCollection200Response](docs/CreateCollection200Response.md)
 - [CreateCollection200ResponseAllOf](docs/CreateCollection200ResponseAllOf.md)
 - [CreateCollection200ResponseAllOfContract](docs/CreateCollection200ResponseAllOfContract.md)
 - [CreateCollection200ResponseAllOfContractAllOf](docs/CreateCollection200ResponseAllOfContractAllOf.md)
 - [CreateCollectionItemRequest](docs/CreateCollectionItemRequest.md)
 - [CreateCollectionItemRequestAttributesInner](docs/CreateCollectionItemRequestAttributesInner.md)
 - [CreateCollectionRequest](docs/CreateCollectionRequest.md)
 - [CreateContractRequest](docs/CreateContractRequest.md)
 - [CreateCurrency200Response](docs/CreateCurrency200Response.md)
 - [CreateCurrencyRequest](docs/CreateCurrencyRequest.md)
 - [CreateGameRequest](docs/CreateGameRequest.md)
 - [CreateLootboxManager200Response](docs/CreateLootboxManager200Response.md)
 - [CreateLootboxManagerRequest](docs/CreateLootboxManagerRequest.md)
 - [CreatePlayerRequest](docs/CreatePlayerRequest.md)
 - [CreateShop200Response](docs/CreateShop200Response.md)
 - [CreateShopRequest](docs/CreateShopRequest.md)
 - [CurrencyModel](docs/CurrencyModel.md)
 - [GameModel](docs/GameModel.md)
 - [GetCollections200ResponseInner](docs/GetCollections200ResponseInner.md)
 - [GetCollections200ResponseInnerAllOf](docs/GetCollections200ResponseInnerAllOf.md)
 - [GetCurrencies200ResponseInner](docs/GetCurrencies200ResponseInner.md)
 - [GetCurrencyFees200Response](docs/GetCurrencyFees200Response.md)
 - [GetLootboxManagers200ResponseInner](docs/GetLootboxManagers200ResponseInner.md)
 - [GetPlayerData200Response](docs/GetPlayerData200Response.md)
 - [GetShops200ResponseInner](docs/GetShops200ResponseInner.md)
 - [GrantCollectionRoleRequest](docs/GrantCollectionRoleRequest.md)
 - [GrantCurrencyRoleRequest](docs/GrantCurrencyRoleRequest.md)
 - [LootboxManagerLootbox](docs/LootboxManagerLootbox.md)
 - [LootboxManagerModel](docs/LootboxManagerModel.md)
 - [MintCollectionItemRequest](docs/MintCollectionItemRequest.md)
 - [MintCurrencyRequest](docs/MintCurrencyRequest.md)
 - [PlayerModel](docs/PlayerModel.md)
 - [PublicGame](docs/PublicGame.md)
 - [PublicPlayer](docs/PublicPlayer.md)
 - [PublicPlayerWallet](docs/PublicPlayerWallet.md)
 - [RevokeCollectionRoleRequest](docs/RevokeCollectionRoleRequest.md)
 - [SetCollectionApprovalRequest](docs/SetCollectionApprovalRequest.md)
 - [SetCollectionItemTimelockRequest](docs/SetCollectionItemTimelockRequest.md)
 - [SetCurrencyFeesRequest](docs/SetCurrencyFeesRequest.md)
 - [SetLootboxManagerLootboxRequest](docs/SetLootboxManagerLootboxRequest.md)
 - [SetPlayerDataRequest](docs/SetPlayerDataRequest.md)
 - [SetShopOfferRequest](docs/SetShopOfferRequest.md)
 - [ShopModel](docs/ShopModel.md)
 - [ShopOffer](docs/ShopOffer.md)
 - [TransactionModel](docs/TransactionModel.md)
 - [TransferCollectionItemRequest](docs/TransferCollectionItemRequest.md)
 - [TransferCurrencyRequest](docs/TransferCurrencyRequest.md)
 - [UpdateGameRequest](docs/UpdateGameRequest.md)
 - [UpdatePlayerRequest](docs/UpdatePlayerRequest.md)
 - [WalletModel](docs/WalletModel.md)
 - [WithdrawFromShopRequest](docs/WithdrawFromShopRequest.md)
 - [WriteContractRequest](docs/WriteContractRequest.md)
 - [WriteContractRequestArgsInner](docs/WriteContractRequestArgsInner.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

metafabproject@gmail.com

