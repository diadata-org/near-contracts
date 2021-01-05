#!/bin/bash
bash build.sh

# MASTER TEST ACCOUNT CREDENTIALS
# {"account_id":"dia-oracles.testnet","public_key":"ed25519:4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT","private_key":"ed25519:2ma7su54FHe4qJs3h5fL4X4iD3ru65Q5bBAqcyzRhjBTsjLdGWgXjd4ZTiP1AxBUzhHBFuQAjrhMAWS2kTYcZsnV"}
# Seed Phrase: drive help tiny office kiss divert autumn normal hill copy dove accident
#

##---------------------
## GATEWAY MAIN CONTRACT
##---------------------
## Use dia-oracles.testnet credentials for test contracts (add-key was called on acc creation)
cp ~/.near-credentials/default/dia-oracles.testnet.json ~/.near-credentials/default/contract.dia-oracles.testnet.json
#create contract.dia-oracles.testnet account
#near add-key contract.dia-oracles.testnet 4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT
#near create-account contract.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy contract.dia-oracles.testnet res/dia_contract.wasm new '{"owner_id":"dia-oracles.testnet"}' 

## deploy no-init
near deploy contract.dia-oracles.testnet res/dia_contract.wasm

##---------------------
## QUOTE TEST CONTRACT
##---------------------
## Use dia-oracles.testnet credentials for test contracts (add-key was called on acc creation)
cp ~/.near-credentials/default/dia-oracles.testnet.json ~/.near-credentials/default/quote-test-client.dia-oracles.testnet.json
## create quote-test-contract account
#near create-account quote-test-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near add-key quote-test-client.dia-oracles.testnet 4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT
#near deploy quote-test-client.dia-oracles.testnet res/quote_test_contract.wasm new '{}'

## deploy no-init
near deploy quote-test-client.dia-oracles.testnet res/quote_test_contract.wasm 

##---------------------
## SUPPLY TEST CONTRACT
##---------------------
## Use dia-oracles.testnet credentials for test contracts (add-key was called on acc creation)
cp ~/.near-credentials/default/dia-oracles.testnet.json ~/.near-credentials/default/supply-test-client.dia-oracles.testnet.json
## create supply-test-contract account
#near create-account supply-test-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near add-key supply-test-client.dia-oracles.testnet 4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT
#near deploy supply-test-client.dia-oracles.testnet res/supply_test_contract.wasm new '{}'

## deploy no-init
near deploy supply-test-client.dia-oracles.testnet res/supply_test_contract.wasm 

##---------------------
## TRADE VOLUME TEST CONTRACT
##---------------------
## Use dia-oracles.testnet credentials for test contracts (add-key was called on acc creation)
cp ~/.near-credentials/default/dia-oracles.testnet.json ~/.near-credentials/default/volume-test-client.dia-oracles.testnet.json
## create volume-test-contract account
#near create-account volume-test-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near add-key volume-test-client.dia-oracles.testnet 4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT
#near deploy volume-test-client.dia-oracles.testnet res/trade_volume_test_contract.wasm new '{}'


## deploy no-init
near deploy volume-test-client.dia-oracles.testnet res/trade_volume_test_contract.wasm 

##---------------------
## SYMBOLS TEST CONTRACT
##---------------------
#Use dia-oracles.testnet credentials for test contracts (add-key was called on acc creation)
cp ~/.near-credentials/default/dia-oracles.testnet.json ~/.near-credentials/default/symbols-test-client.dia-oracles.testnet.json
#create symbols-test-contract account
#near create-account symbols-test-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near add-key symbols-test-client.dia-oracles.testnet 4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT
#near deploy symbols-test-client.dia-oracles.testnet res/symbols_test_contract.wasm new '{}'

## deploy no-init
near deploy symbols-test-client.dia-oracles.testnet res/symbols_test_contract.wasm 
