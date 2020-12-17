# DIA-NEAR integration

## DIA Smart Contract

The DIA smart contract for NEAR (dia-sc) deployed on `dia.oracles.near` allows any NEAR smart contract to request [DIA oracles data](https://diadata.org/).

The main contract API point is the `dia.oracles.near.request()` function. The parameters to the request function include: 
* requestring-contract-account-id
* A contract-specific request id
* the data-key requested (defines the DIA API to request)
* the data-item requested (filter the DIA API result to return)
* the callback method where the data will be received.

The data is retrieved by an external process called `dia-adapter` and then sent back to the contract requesting the data by a call to the specified callback function. e.g.
`near call [requestring-contract-account-id] [callback-method] { "err":"", "data":[dia-api-call-result] }

## Example

TODO - example code for a request quote-BTC request and example callback-response

## Technical details

This smart contract stores internally pending requests with the following format:
* originating contract account id (String)
* request id (U128)
* requested data-key (String)
* data-item (String)
* callback method (String)

`dia.oracles.near` has a method to report how many pending requests there are: `dia.oracles.near.get_pending_requests_count()` and another to read all existing requests: `  dia.oracles.near.get_pending_requests()`

`dia.oracles.near` has owner’s method to remove pending request (when the request is completed): `dia.oracles.near.remove({contract_id:string, request_id:U128})`

The `dia-adapter` is periodically polling the `dia.oracles.near` smart contract by using `get_pending_requests` and will react to pending requests by:
* Query the corresponding DIA API endpoint for the data
* Prepare and execute a NEAR transaction with a function call to the originating contract’s callback method, including the the contract-specific request ID, the data-key, data-item and the data information retrieved from the DIA API endpoint
* The transaction will be signed by `dia.oracles.near` so the originating contract can verify the data source by controlling `env::signer_account_id` in the callback
* `dia-adapter` records the request contract+ID as serviced and calls `dia.oracles.near.remove()` to remove the request from the pending list

## Code Examples
TODO

## Deploy Instructions

* Execute `deploy.sh`, this will delete the sub-account to erase state, re-create it and deploy the contract.
