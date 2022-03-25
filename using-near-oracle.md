# Access DIA Near oracle

DIA near oracles is deplyed at contract.diadata.near, to get the quotation from oracles calle contract need to request quotation fro DIA near oracles with near as fee per call.


DIA Near oracle in return will give quotation as response which can be consumed by callee contract.



### Make Request to DIA Near oracle contract

```


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
````

After making request DIA Oracle call the callback with the quotation response, make sure you create a callback function here is example for it


````

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

```


Call back function recieves the response for which can be consumed as per the requirement if contract.


