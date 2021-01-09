# DIA-NEAR integration

## DIA Smart Contract

The DIA smart contract for NEAR (dia-sc) deployed on `contract.dia.oracles.near` allows any NEAR smart contract to request [DIA oracles data](https://diadata.org/).

The main contract API endpoint is the `contract.dia.oracles.near.request()` function. The parameters to the request function include: 
* A contract-specific request id.
* the data-key requested (defines the DIA API to request). 
* the data-item requested (filters the DIA API result to return). 
* the callback method where the data will be received.

The data is retrieved by an external process called [dia-adapter](https://github.com/Narwallets/dia-adapter) and sent back to the contract requesting the data, by calling the callback function specified in the request. e.g.
`near call [requestring-contract-account-id] [callback-method] { "err":"", "data":[dia-api-call-result-data] }`

## Example Contracts

There are four working-example contracts in this repository you can use as a starting point, each one showing how to consume a specific DIADATA API:
* [Quotation Test Contract](quote-test-contract/src/lib.rs)
* [Suply Test Contract](supply-test-contract/src/lib.rs)
* [Volume Test Contract](trade-volume-test-contract/src/lib.rs)
* [Symbols Test Contract](symbols-test-contract/src/lib.rs)

All the DIADATA Apis are documented [here](https://docs.diadata.org/documentation/api-1)


## Technical details

The main smart contract `contract.dia.oracles.near` stores internally pending requests with the following format:
* originating contract account id (String)
* request id (U128)
* requested data-key (String)
* data-item (String)
* callback method (String)

The main gaeway contract has a method to report how many pending requests there are: `get_pending_requests_count()` and another to read all existing requests: `get_pending_requests() -> Vec<RequestInfo>`

`contract.dia.oracles.near` has an owner’s method to remove pending request (once the request is completed): `remove({contract_id:string, request_id:U128})`

The `dia-adapter` is periodically polling the `dia.oracles.near` smart contract by using `get_pending_requests_count()` and will react to pending requests by:
* Querying the corresponding DIA API endpoint for the data
* Prepare and execute a NEAR transaction with a function call to the originating contract’s callback method, including the contract-specific request ID, the data-key, data-item and the data information retrieved from the DIA API endpoint
* The transaction will be signed by `dia.oracles.near` so the originating contract can verify the data source by controlling `env::signer_account_id` in the callback
* `dia-adapter` records the request contract+ID as serviced and calls `dia.oracles.near.remove()` to remove the request from the pending list

## Build Instructions

* To build all the contracts (Gateway main contract and test contracts) run `build.sh`, built contracts will be copied into the `./res` dir of this proyect.

## Unit Test Instructions

* To run all unit tests (of the Gateway main contract and test contracts) run `test.sh`.

## Preparing Integration Tests Instructions

The following steps are included in the file [deploy-testnet.sh](deploy-testnet.sh)

* First you will need to create a Near account for the gateway contract and the test contracts.
* We will use the near CLI for this example.
* Create a Near account and then login into it with `near login`, repeat this for each account.
* Execute build.sh, the built contracts will be copied into the 'res' folder of this proyect.
* Use `near deploy <account> <path to the contract>` eg.
`near deploy contract.diadata.testnet res/dia_contract.wasm`
* Before using the main contract it has to be initialized with:
`near call contract.diadata.testnet new '{"owner_id":"diadata.testnet"}'  --accountId diadata.testnet`
    Look how we used a sub-account to deploy the contract, in case the testing contract's state has to be deleted the sub-account can be deleted and re-created since re-deploying will preserve the state.
* Test contracts have to be initialized with:
`near call tc1.diadata-test.testnet new --accountId diadata-test.testnet`
    Test contracts are initialized with a request id that is incremented for every request and can be set/obtain using the `set_id` and `get_id` functions of the contract.

Once all the contracts are deployed in testnet, you can run the integration test from the [dia-adapter repostory](https://github.com/Narwallets/dia-adapter)

## DIADATA Testnet Infrastructure
In order to facilitate developing and testing, we will have a running version of the infraestructure in testnet at the near address `contract.dia.oracles.testnet`, and our own server running `near-adapter` to process pending requests.

All the client-contract examples run against this test infraestructure.

You can also use this infraestruture for testing at testnet.

While testing, you can use `near view contract.dia.oracles.testnet get_pending_requests` to check all pending requests (including other contracts using the infrastructure) at any time.
