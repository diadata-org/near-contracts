use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::payable;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

const DEPOSIT_FOR_REQUEST: u64 = 0; /* Amount that clients have to pay to call make a request to the api */

/// Request dto, same data structure used for storage and sharing
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Request {
    pub contract_account_id: String, /* Originating contract account id */
    pub request_id: U128, /* Originating contract specific id */
    pub data_key: String, /* Dia api to request */
    pub data_item: String, /* Data to filter the requested result */
    pub callback: String, /* Endpoint where data will be received */
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DiaApiGatewayContract {
    /// The Account Id of the owner of the contract
    pub owner_id: AccountId,
    /// Persistent storage of the requests, completed requests are deleted
    pub requests: Vec<Request>
}

impl Default for DiaApiGatewayContract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")
    }
}

#[near_bindgen]
impl DiaApiGatewayContract {
    /// Initializes the contract with the given owner_id
    #[init]
    pub fn new(
        owner_id: AccountId
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        return Self {
            owner_id,
            requests: Vec::new()
        };
    }

    /******************/
    /* Client methods */
    /******************/
    #[payable]
    pub fn request(&mut self, request_id: U128, data_key: String, data_item: String, callback: String){
        /* Check that deposit (in yocto-near) is enough */
        let attached_deposit = env::attached_deposit();
        assert_eq!(attached_deposit >= DEPOSIT_FOR_REQUEST,
        "The required attached deposit is {}, but the given attached deposit is is {}",
        DEPOSIT_FOR_REQUEST,
        attached_deposit
    ); 
        let request = Request{
            contract_account_id: env::signer_account_id(),
            request_id,
            data_key,
            data_item,
            callback: format!("{}{}", callback, test)
        };
        return self.requests.push(request)
    }


    /***********************/
    /* Dia adapter methods */
    /***********************/

    pub fn get_pending_requests_count(&self)-> u64{
        return self.requests.len() as u64
    }

    pub fn get_pending_requests(&self)-> Vec<Request>{
        return self.requests.clone()
    }

    pub fn remove(&mut self, contract_id: String, request_id: U128){
        let index = self.requests.iter().position(|request| {
            request.request_id == request_id && request.contract_account_id == contract_id
        }).unwrap();
        self.requests.remove(index);
    }
}


