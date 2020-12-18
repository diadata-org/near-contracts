#!/bin/bash

# TEST ACCOUNT CREDENTIALS
# {"account_id":"dia-oracles.testnet","public_key":"ed25519:4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT","private_key":"ed25519:2ma7su54FHe4qJs3h5fL4X4iD3ru65Q5bBAqcyzRhjBTsjLdGWgXjd4ZTiP1AxBUzhHBFuQAjrhMAWS2kTYcZsnV"}
# Seed Phrase: drive help tiny office kiss divert autumn normal hill copy dove accident
#

#GATEWAY CONTRACT
#create contract.dia-oracles.testnet account
#near create-account contract.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy contract.dia-oracles.testnet res/dia_contract.wasm new '{"owner_id":"dia-oracles.testnet"}' 

#TEST CONTRACT
#create quote-test-contract account
#near create-account quote-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy quote-client.dia-oracles.testnet res/quote_test_contract.wasm new '{"owner_id":"dia-oracles.testnet"}' 

## deploy no-init
near deploy quote-client.dia-oracles.testnet res/quote_test_contract.wasm 

