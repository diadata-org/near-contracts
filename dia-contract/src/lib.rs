use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};
use near_sdk::{env, near_bindgen, AccountId};


#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;
const ONE_NEAR_CENT:u128 = ONE_NEAR/100;
const DEPOSIT_FOR_REQUEST: u128 = ONE_NEAR_CENT; // amount that clients have to attach to make a request to the api

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
        if DEPOSIT_FOR_REQUEST>0 {
            let attached_deposit = env::attached_deposit();
            assert!(attached_deposit >= DEPOSIT_FOR_REQUEST,
                "The required attached deposit is {}, but the given attached deposit is {}",
                DEPOSIT_FOR_REQUEST,
                attached_deposit
            ); 
        }
        let request = Request{
            contract_account_id: env::predecessor_account_id(),
            request_id,
            data_key,
            data_item,
            callback: callback
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
        /* Prevent other people from removing pending requests */
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
        let index = self.requests.iter().position(|request| {
            request.request_id == request_id && request.contract_account_id == contract_id
        }).unwrap();
        self.requests.remove(index);
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


    static OWNER: &str = "testowner.testnet";
    static TEST_ACCOUNT: &str = "client.testnet";

    /// Set the contract context
    pub fn initialize(){
        let context = get_context(String::from(TEST_ACCOUNT), 10);                    
        testing_env!(context); 
    }
    
    /// Creates a contract
    pub fn create_contract() -> DiaApiGatewayContract{
        let contract = super::DiaApiGatewayContract::new(String::from(OWNER));
        return contract;
    }

    ///Creates a request as a client and returns the expected saved value
    pub fn create_request(contract: &mut DiaApiGatewayContract) -> Request{
        contract.request(U128::from(1231223), String::from("quotation"), String::from("BTC"), String::from("callback"));
        let expected_request = Request{
            contract_account_id: String::from(TEST_ACCOUNT),
            request_id: U128::from(1231223),
            data_key: String::from("quotation"),
            data_item: String::from("BTC"),
            callback: String::from("callback")
        };
        return expected_request;
    }

    /// Defines the context for the contract
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: OWNER.to_string(),
            signer_account_id: OWNER.to_string(),
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

    #[test]
    fn test_creation() {
        initialize();
        let contract = create_contract();
        assert_eq!(contract.owner_id, String::from(OWNER), "Owner is different from the expected");
    }

    #[test]
    #[should_panic]
    fn test_default_creation() {
        initialize();
        <DiaApiGatewayContract as Default>::default();
    }

    #[test]
    fn test_client_methods(){
        initialize();
        let mut contract = create_contract();
        let expected_request = create_request(&mut contract);
        
        if let Some(request) = contract.requests.get(0) {
            assert_eq!(expected_request, *request, "Saved request has wrong field values");
        }
        else{
            panic!("Request not saved");
        }
    }

    #[test]
    fn test_adapter_methods(){
        initialize();
        /* Create a request as a client */
        let mut contract = create_contract();
        let expected_request = create_request(&mut contract);
        /* Change context to match a method called by the adapter */
        let context = get_context(String::from(OWNER), 10);                    
        testing_env!(context); 
      
        println!("Testing 'get_pending_requests_count' method");
        let pending_requests_count = contract.get_pending_requests_count();
        assert_eq!(pending_requests_count, 1 as u64, "Wrong value ({}) in pending requests", pending_requests_count);

        println!("Testing 'get_pending_requests' method");
        let pending_requests = contract.get_pending_requests();
        if let Some(request) = pending_requests.get(0) {
            assert_eq!(expected_request, *request, "Method 'get_pending_requests' returns wrong data");
        }
        else{
            panic!("Method 'get_pending_requests' dont returns data");
        }

        println!("Testing 'remove' method");
        contract.remove(String::from(TEST_ACCOUNT), U128::from(1231223));
    }
    
}


