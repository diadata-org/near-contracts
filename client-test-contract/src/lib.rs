use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Balance, Gas};
use std::fmt::Write;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

const DEPOSIT_FOR_REQUEST: Balance = 0; /* Amount that clients have to pay to call make a request to the api */
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;
const DIA_GATEWAY_ACCOUNT_ID: &str = "test.dia-sc.testnet";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ClientTestContract {
    pub contract_id: String,
    pub callback_response: Response
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: String,
    data_key: String,
    data_item: String,
    callback: String
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ResponseData{
    Quote(QuoteData),
    None
}

#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    err: String,
    data: ResponseData,
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct QuoteData {
    symbol: String,
    name: String,
    price: f64,
    price_yesterday: f64,
    volume_yesterday_usd: f64,
    source: String,
    time: String,
    itin: String,
    signer_account_id: String
}

impl Default for ClientTestContract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")  
    }
}

#[near_bindgen]
impl ClientTestContract {

    ///Initialize the contract with a random id
    #[init]
    pub fn new()-> Self{
        /* Prevent re-initializations */
        assert!(!env::state_exists(), "This contract is already initialized");

         /* Get a random seed to create an unique id for the contract */
         let seed = &env::random_seed()[..7];
         /* Convert to hex string */
         let mut random_id = String::new();
         for val in seed{
             write!(&mut random_id, "{:x?}", val).expect("Unable to write")
         }
         return Self {
             contract_id: random_id,
             callback_response: Response{
                 err: String::new(),
                 data: ResponseData::None
             }
         };
    }


    /******************/
    /* Config methods */
    /******************/
    
    ///Set contract_id to a supplied value
    pub fn set_id(&mut self, contract_id: String)-> String{
        self.contract_id = contract_id;
        return self.contract_id.clone();
    }

    ///Get contract_id
    pub fn get_id(&self)-> String{
        return self.contract_id.clone();
    }

    /****************/
    /* Test methods */
    /****************/

    ///Make a request to the dia-gateway smart contract
    #[payable]
    pub fn make_request(&mut self, data_key: String, data_item: String)-> near_sdk::Promise{

        return near_sdk::Promise::new(String::from(DIA_GATEWAY_ACCOUNT_ID)).function_call(
            b"request".to_vec(),
            near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
                request_id: self.contract_id.clone(),
                data_key: data_key,
                data_item: data_item,
                callback: String::from("callback")
            }).unwrap(),
            DEPOSIT_FOR_REQUEST,
            GAS_FOR_REQUEST
        );
    }

    ///View the dia-adapter response to the contract's request
    pub fn get_callback_response(&self)-> Response{
        return self.callback_response.clone();
    }


    /***********************/
    /* Dia adapter methods */
    /***********************/

    ///Callback that the dia-adapter uses
    pub fn callback(&mut self, err: String, response: ResponseData){
        self.callback_response = Response {
            err: err,
            data: response
        };
    }

}


