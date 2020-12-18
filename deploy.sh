#!/bin/bash

# TEST ACCOUNT CREDENTIALS
# {"account_id":"dia-oracles.testnet","public_key":"ed25519:4rW9mncNoTvEGhFLgmcak5gbmqcL5Y5v89FcN23QujsT","private_key":"ed25519:2ma7su54FHe4qJs3h5fL4X4iD3ru65Q5bBAqcyzRhjBTsjLdGWgXjd4ZTiP1AxBUzhHBFuQAjrhMAWS2kTYcZsnV"}
# Seed Phrase: drive help tiny office kiss divert autumn normal hill copy dove accident
#

##---------------------
## GATEWAY MAIN CONTRACT
##---------------------
#create contract.dia-oracles.testnet account
#near create-account contract.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy contract.dia-oracles.testnet res/dia_contract.wasm new '{"owner_id":"dia-oracles.testnet"}' 

## deploy no-init
near deploy contract.dia-oracles.testnet res/dia_contract.wasm

##---------------------
## QUOTE TEST CONTRACT
##---------------------
#create quote-test-contract account
#near create-account quote-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy quote-client.dia-oracles.testnet res/quote_test_contract.wasm

## deploy no-init
near deploy quote-client.dia-oracles.testnet res/quote_test_contract.wasm 

##---------------------
## SUPPLY TEST CONTRACT
##---------------------
#create supply-test-contract account
#near create-account supply-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy supply-client.dia-oracles.testnet res/supply_test_contract.wasm new
# {"account_id":"supply-client.dia-oracles.testnet","public_key":"ed25519:F4KarCDxQ6kZmrhmTcmBaa7AQ8jEQmpGJ8Pf3LFwKzAb","private_key":"ed25519:2yWosCmw4WNMmNqt4wtCb2dpvYekA11bbPTHirfZoHzPUp4qa5gbczK7wtPo3Lvp4CRJnCrgE7rY7i3Ciu48inQX"}

## deploy no-init
near deploy supply-client.dia-oracles.testnet res/supply_test_contract.wasm 

##---------------------
## TRADE VOLUME TEST CONTRACT
##---------------------
#create trade-volume-test-contract account
#near create-account volume-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy volume-client.dia-oracles.testnet res/trade_volume_test_contract.wasm new
# {"account_id":"volume-client.dia-oracles.testnet","public_key":"ed25519:EKLWR2VFNiaXQjxff8RDVSMgF79oB8LNkxXrttUfiY58","private_key":"ed25519:24JeQFyTNAGnpfbPzkyxwtVW9JHTXgYTebd69N2EYiWjSQooUhRH5wHztm5FPx7w6zcGxPhZk79xetixUctda2st"}

## deploy no-init
near deploy volume-client.dia-oracles.testnet res/trade_volume_test_contract.wasm 

##---------------------
## SYMBOLS TEST CONTRACT
##---------------------
#create symbols-test-contract account
#near create-account symbols-client.dia-oracles.testnet --masterAccount dia-oracles.testnet
#near deploy symbols-client.dia-oracles.testnet res/symbols_volume_test_contract.wasm new
# {"account_id":"symbols-client.dia-oracles.testnet","public_key":"ed25519:HKctk4N86tiqATp5xkS4mGeqTykVYFAav7yUqjca9pyL","private_key":"ed25519:U3TncxNnZA9f8KUW8JcBr9awfaqEPUNF13hwsAApYDDygkYCLiGPfW5hZAtQzp871wezrLUaJB3upfqQhEjtavr"}

## deploy no-init
near deploy symbols-client.dia-oracles.testnet res/symbols_volume_test_contract.wasm 
