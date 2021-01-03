# DIA-NEAR integration

## DIA Smart Contract

The DIA smart contract for NEAR (dia-sc) deployed on `dia.oracles.near` allows any NEAR smart contract to request [DIA oracles data](https://diadata.org/).

The main contract API point is the `dia.oracles.near.request()` function. The parameters to the request function include: 
* A contract-specific request id.
* the data-key requested (defines the DIA API to request). 
* the data-item requested (filter the DIA API result to return). 
* the callback method where the data will be received.

The data is retrieved by an external process called `dia-adapter` and then sent back to the contract requesting the data by a call to the specified callback function. e.g.
`near call [requestring-contract-account-id] [callback-method] { "err":"", "data":[dia-api-call-result] }

## Example

* Calling dia.oracles.near.request() with api 'QUOTE' and item 'BTC' from a contract:

`
use near_sdk::serde::{Serialize};
use near_sdk::{env, Balance, Gas};
use near_sdk::json_types::{U128};

const DEPOSIT_FOR_REQUEST: Balance = 1; /* Example amount that clients have to pay to make a request to the api */
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;

/// Enum declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ResponseData{
    Quote (QuoteData),
    None
}

/// Struct declaring callback data to parse the response
#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    err: String,
    data: ResponseData,
}

/// Struct declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[allow(non_snake_case)]
pub struct QuoteData {
    Symbol: String,
    Name: String,
    Price: f64,
    PriceYesterday: f64,
    VolumeYesterdayUSD: f64,
    Source: String,
    Time: String,
    ITIN: String
}

/// Struct declaring the request function parameters
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: U128,
    data_key: String,
    data_item: String,
    callback: String
}

/// The function that makes the request
pub fn make_request(){
    
    near_sdk::Promise::new(String::from("contract.dia-oracles.testnet")).function_call(
        b"request".to_vec(),
        near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
            request_id: U128::from(2354576434),
            data_key: String::from("quote"),
            data_item: String::from("btc"),
            callback: String::from("callback")
        }).unwrap(),
        DEPOSIT_FOR_REQUEST,
        GAS_FOR_REQUEST
    );
}

/// The function that will receive the requested information
pub fn callback(&mut self, err: String, data: ResponseData){
    //verify data origin
    assert!(env::signer_account_id() == "dia-oracles.testnet");
    //use quote
    match &data {
        ResponseData::None => env::log("empty data".as_bytes()),
        ResponseData::Quote(x)=>env::log(format!("Quote {} {}",x.Name,x.Price).as_bytes())
    }
    //store last response
    self.callback_response = Response {
        err: err,
        data: data
    };
}
`

* Calling dia.oracles.near.request() with api 'SYMBOLS' with item 'Kraken' from a contract:

`
use near_sdk::serde::{Serialize};
use near_sdk::{env, Balance, Gas};
use near_sdk::json_types::{U128};

const DEPOSIT_FOR_REQUEST: Balance = 1; /* Example amount that clients have to pay to make a request to the api */
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;

/// Struct declaring the request function parameters
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: U128,
    data_key: String,
    data_item: String,
    callback: String
}

/// Enum declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ResponseData{
    Symbols (SymbolsData),
    None
}

/// Struct declaring callback data to parse the response
#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    err: String,
    data: ResponseData,
}

/// Struct declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[allow(non_snake_case)]
pub struct SymbolsData {
    Symbols: Vec<String>
}

/// The function that makes the request
pub fn make_request(){
    
    near_sdk::Promise::new(String::from("contract.dia-oracles.testnet")).function_call(
        b"request".to_vec(),
        near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
            request_id: U128::from(2354576434),
            data_key: String::from("symbols"),
            data_item: String::from("Kraken"),
            callback: String::from("callback")
        }).unwrap(),
        DEPOSIT_FOR_REQUEST,
        GAS_FOR_REQUEST
    );
}

/// The function that will receive the requested information
pub fn callback(&mut self, err: String, data: ResponseData){
    //verify data origin
    assert!(env::signer_account_id() == "dia-oracles.testnet");
    //use symbols
    match &data {
        ResponseData::None => env::log("empty data".as_bytes()),
        ResponseData::Symbols(x)=>env::log(format!("Symbols {:?}", x.Symbols).as_bytes())
    }
    //store last response
    self.callback_response = Response {
        err: err,
        data: data
    };
}
`

* Calling dia.oracles.near.request() with api 'SUPPLY' with item 'BTC' from a contract:


`
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Balance, Gas};
use near_sdk::json_types::{U128};

pub const DEPOSIT_FOR_REQUEST: Balance = 0; /* Amount that clients have to pay to call make a request to the api */
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;

/// Struct declaring the request function parameters
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: U128,
    data_key: String,
    data_item: String,
    callback: String
}

/// Enum declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ResponseData{
    Supply (SupplyData),
    None
}

/// Struct declaring callback data to parse the response
#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    err: String,
    data: ResponseData,
}

/// Struct declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[allow(non_snake_case)]
pub struct SupplyData {
    Symbol: String,
    Name: String,
    CirculatingSupply: u64,
    Source: String,
    Time: String,
    Block: u64
}

/// The function that makes the request
#[payable]
pub fn make_request(&mut self, data_item: String)-> near_sdk::Promise{

    self.request_id+=1;

    return near_sdk::Promise::new(String::from(DIA_GATEWAY_ACCOUNT_ID)).function_call(
        b"request".to_vec(),
        near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
            request_id: U128::from(self.request_id),
            data_key: String::from("supply"),
            data_item: data_item,
            callback: String::from("callback")
        }).unwrap(),
        DEPOSIT_FOR_REQUEST,
        GAS_FOR_REQUEST
    );
}

/// The function that will receive the requested information
pub fn callback(&mut self, err: String, data: ResponseData){
    //verify data origin
    assert!(env::signer_account_id() == SIGNER_DIA_ORACLES_ACCOUNT_ID);
    //use supply
    match &data {
        ResponseData::None => env::log("empty data".as_bytes()),
        ResponseData::Supply(x)=>env::log(format!("Supply {} {}",x.Name,x.CirculatingSupply).as_bytes())
    }
    //store last response
    self.callback_response = Response {
        err: err,
        data: data
    };
}
`

* Calling dia.oracles.near.request() with api 'VOLUME' with item 'BTC' and optional parameters  'starttime' and 'endtime' from a contract:

where data_item is: 'BTC?starttime=1609712821&endtime=1609713821'


`
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Balance, Gas};
use near_sdk::json_types::{U128};

pub const DEPOSIT_FOR_REQUEST: Balance = 0; /* Amount that clients have to pay to call make a request to the api */
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;

/// Struct declaring the request function parameters
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: U128,
    data_key: String,
    data_item: String,
    callback: String
}

/// Enum declaring callback data to parse the response
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ResponseData{
    TradeVolume (TradeVolumeData),
    None
}

/// Struct declaring callback data to parse the response
#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    err: String,
    data: ResponseData,
}

/// Type declaring callback data to parse the response
pub type TradeVolumeData = f64;

/// The function that makes the request
#[payable]
pub fn make_request(&mut self, data_item: String)-> near_sdk::Promise{

    self.request_id+=1;

    return near_sdk::Promise::new(String::from(DIA_GATEWAY_ACCOUNT_ID)).function_call(
        b"request".to_vec(),
        near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
           request_id: U128::from(self.request_id),
            data_key: String::from("volume"),
            data_item: data_item,
            callback: String::from("callback")
        }).unwrap(),
        DEPOSIT_FOR_REQUEST,
        GAS_FOR_REQUEST
    );
}

/// The function that will receive the requested information
pub fn callback(&mut self, err: String, data: ResponseData){
    //verify data origin
    assert!(env::signer_account_id() == SIGNER_DIA_ORACLES_ACCOUNT_ID);
    //use trade volume
    match &data {
        ResponseData::None => env::log("empty data".as_bytes()),
        ResponseData::TradeVolume(x)=>env::log(format!("Trade volume {}", x).as_bytes())
    }
    //store last response
    self.callback_response = Response {
        err: err,
        data: data
    };
}
`

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
* Prepare and execute a NEAR transaction with a function call to the originating contract’s callback method, including the contract-specific request ID, the data-key, data-item and the data information retrieved from the DIA API endpoint
* The transaction will be signed by `dia.oracles.near` so the originating contract can verify the data source by controlling `env::signer_account_id` in the callback
* `dia-adapter` records the request contract+ID as serviced and calls `dia.oracles.near.remove()` to remove the request from the pending list

## Build Instructions

* To build all the contracts (Gateway main contract and test contracts) run 'build.sh', built contracts will be copied into the 'res' folder of this proyect.

## Test Instructions

* To run all unit tests (of the Gateway main contract and test contracts) run 'test.sh'.

## Test Deploy Instructions

* First you will need to create a Near account for the gateway contract and the test contracts.
* We will use the near CLI for this example.
* Create a Near account and then login into it with `near login`, repeat this for each account.
* Execute build.sh, the built contracts will be copied into the 'res' folder of this proyect.
* Use `near deploy <account> <path to the contract>` eg.
`near deploy contract.diadata.testnet res/dia_contract.wasm`
* Before using the contract it has to be initialized with:
`near call contract.diadata.testnet new '{"owner_id":"diadata.testnet"}'  --accountId diadata.testnet`
    Look how we used a sub-account to deploy the contract, in case the contract's state has to be deleted the sub-account can be deleted and created again since re-deploying will preserve the state.
* Test contracts have to be initialized with:
`near call tc1.diadata-test.testnet new --accountId diadata-test.testnet`
    Test contracts are initialized with a request id that is incremented for every request and can be set/obtain using the 'set_id' and 'get_id' functions of the contract.


