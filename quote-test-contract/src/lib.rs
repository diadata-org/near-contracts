use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, Balance, Gas};
use near_sdk::json_types::{U128};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

const DEPOSIT_FOR_REQUEST: Balance = 0; /* Amount that clients have to pay to call make a request to the api */
const GAS_FOR_REQUEST: Gas = 50_000_000_000_000;
const DIA_GATEWAY_ACCOUNT_ID: &str = "contract.dia-oracles.testnet";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct QuoteTestContract {
    pub request_id: u128,
    pub callback_response: Response
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: U128,
    data_key: String,
    data_item: String,
    callback: String
}

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ResponseData{
    Quote (QuoteData),
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

impl Default for QuoteTestContract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")  
    }
}

#[near_bindgen]
impl QuoteTestContract {

    ///Initialize the contract with a random id
    #[init]
    pub fn new()-> Self{
        /* Prevent re-initializations */
        assert!(!env::state_exists(), "This contract is already initialized");
        return Self {
             request_id: 100,
             callback_response: Response{
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
        self.request_id = request_id.0;
    }

    ///Get request_id
    pub fn get_id(&self)-> U128{
        return self.request_id.into();
    }

    /****************/
    /* Test methods */
    /****************/

    ///Make a request to the dia-gateway smart contract
    #[payable]
    pub fn make_request(&mut self, data_key: String, data_item: String)-> near_sdk::Promise{

        self.request_id+=1;

        return near_sdk::Promise::new(String::from(DIA_GATEWAY_ACCOUNT_ID)).function_call(
            b"request".to_vec(),
            near_sdk::serde_json::to_vec(&DiaGatewayRequestArgs {
                request_id: U128::from(self.request_id),
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
    ///Callback to receive dia-api data
    pub fn callback(&mut self, err: String, data: ResponseData){
        //verify data origin
        assert!(env::predecessor_account_id()==DIA_GATEWAY_ACCOUNT_ID);
        //use quote
        match &data {
            ResponseData::None => env::log("empty data".as_bytes()),
            ResponseData::Quote(x)=>env::log(format!("Quote {} {}",x.name,x.price).as_bytes())
        }
        //store last response
        self.callback_response = Response {
            err: err,
            data: data
        };
    }

}

