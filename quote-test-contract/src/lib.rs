//
// QUOTATION DIAAPI TEST CONTRACT/EXAMPLE
//
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Gas};
use near_sdk::json_types::{U128};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;
const ONE_NEAR_CENT:u128 = ONE_NEAR/100;
const DEPOSIT_FOR_REQUEST: u128 = ONE_NEAR_CENT; // amount that clients have to attach to make a request to the api
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;
const DIA_GATEWAY_ACCOUNT_ID: &str = "contract.dia.oracles.testnet";
const SIGNER_DIA_ORACLES_ACCOUNT_ID:&str  = "dia.oracles.testnet";

//contract state
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QuoteTestContract {
    //current request id
    pub current_request_id: u128,
    //last response received
    pub last_callback_response: Response
}

///standard request format
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: U128,
    data_key: String,
    data_item: String,
    callback: String
}

/// Enum declaring diadata-api "quotation" response as optional
#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ResponseData{
    Quote (QuoteData),
    None
}

///standard response format
#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    request_id: U128,
    err: String,
    data: ResponseData,
}

/// Struct declaring diadata-api response model
// https://docs.diadata.org/documentation/api-1/api-endpoints#quotation
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

impl Default for QuoteTestContract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")  
    }
}

#[near_bindgen]
impl QuoteTestContract {

    #[init]
    pub fn new()-> Self{
        /* Prevent re-initializations */
        assert!(!env::state_exists(), "This contract is already initialized");
        return Self {
             current_request_id: 100,
             last_callback_response: Response{
                 request_id: 0.into(),
                 err: String::new(),
                 data: ResponseData::None
             }
         };
    }


    /******************/
    /* Config methods */
    /******************/
    
    ///Set request_id to a supplied value
    pub fn set_id(&mut self, request_id: U128){
        self.current_request_id = request_id.0;
    }

    ///Get request_id
    pub fn get_id(&self)-> U128{
        return self.current_request_id.into();
    }

    /****************/
    /* Main methods */
    /****************/

    ///Make a request to the dia-gateway smart contract
    #[payable]
    pub fn make_request(&mut self, data_item: String)-> near_sdk::Promise{

        self.current_request_id+=1;

        return near_sdk::Promise::new(String::from(DIA_GATEWAY_ACCOUNT_ID)).function_call(
            b"request".to_vec(),
            near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
                request_id: U128::from(self.current_request_id),
                data_key: String::from("quotation"),
                data_item: data_item,
                callback: String::from("callback")
            }).unwrap(),
            DEPOSIT_FOR_REQUEST,
            GAS_FOR_REQUEST
        );
    }

    ///View the dia-adapter response to the contract's request
    pub fn get_callback_response(&self)-> Response{
        return self.last_callback_response.clone();
    }

    ///Clear cached response
    pub fn clear_callback_response(&mut self) {
        self.last_callback_response.data = ResponseData::None; //Clear
    }

    /************************/
    /* Dia adapter callback */
    /************************/
    ///Callback to receive dia-api data
    pub fn callback(&mut self, request_id: U128, err: String, data: ResponseData){
        //verify data origin
        assert!(env::signer_account_id() == SIGNER_DIA_ORACLES_ACCOUNT_ID);
        //check for errrors in the request
        assert!(err.len()==0,err);
        //use quote
        match &data {
            ResponseData::None => env::log("empty data".as_bytes()),
            ResponseData::Quote(x)=>env::log(format!("Quote {} {}",x.Name,x.Price).as_bytes())
        }
        //store last response
        self.last_callback_response = Response {
            request_id: request_id,
            err: err,
            data: data
        };
    }

}

/**************/
/* Unit tests */
/**************/

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};


    /// Set the contract context
    pub fn initialize() {
        let context = get_context(String::from("client.testnet"), 10);                    
        testing_env!(context); 
    }

    /// Defines the context for the contract
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "dia.oracles.testnet".to_string(),
            signer_account_id: "dia.oracles.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    ///Test get_id and set_id methods
    #[test]
    fn test_id() {
        initialize();
        /* Initialize contract */
        let mut contract = super::QuoteTestContract::new();
        let id: U128 = 13123123.into();
        contract.set_id(id.clone());
        assert_eq!(contract.get_id(), id, "Contract id is different from the expected");
    }
}
