use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize};
use near_sdk::{env, near_bindgen, Balance, Gas, AccountId};
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
    pub callbackResponse: Response
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DiaGatewayRequestArgs {
    request_id: String,
    data_key: String,
    data_item: String,
    callback: String
}

#[derive(Serialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Response{
    request_id: String,
    data_key: String,
    data_item: String,
    data_point: String,
    signer_account_id: String
}

impl Default for Response{
    fn default()-> Self {
        return Self {
            request_id: String::new(),
            data_key: String::new(),
            data_item: String::new(),
            data_point: String::new(),
            signer_account_id: String::new()
        }
    }
}

impl Default for ClientTestContract {
    fn default() -> Self {
        // let seed = env::random_seed();
        // /* Convert to hex string */
        // let mut value = String::new();
        // write!(&mut value, "{:x?}", seed).expect("Unable to write");
        return Self {
            contract_id: String::from("asddsadsaa"),
            callbackResponse: Default::default()
        };
    }
}

#[near_bindgen]
impl ClientTestContract {
    
    ///Set contract_id to a supplied value
    pub fn setId(&mut self, contract_id: String)-> String{
        self.contract_id = contract_id;
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

    ///Callback that the dia-adapter uses
    pub fn callback(&mut self, request_id: String, data_key: String, data_item: String, data_point: String, signer_account_id: String){
        self.callbackResponse.request_id = request_id;
        self.callbackResponse.data_key = data_key;
        self.callbackResponse.data_item = data_item;
        self.callbackResponse.data_point = data_point;
        self.callbackResponse.signer_account_id = signer_account_id;
    }

    ///View the dia-adapter response to the contract's request
    pub fn get_callback_response(&self)-> Response{
        return self.callbackResponse.clone();
    }

}


