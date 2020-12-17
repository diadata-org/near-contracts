#!/bin/bash

near delete test.dia-sc.testnet dia-sc.testnet
near create-account test.dia-sc.testnet --masterAccount dia-sc.testnet
near deploy test.dia-sc.testnet res/dia_contract.wasm new '{"owner_id":"dia-sc.testnet"}' 34761560371601