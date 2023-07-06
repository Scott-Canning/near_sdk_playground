## Deploy commands

Recycle account: 
```
near delete contract.scxtt.testnet scxtt.testnet                                                 
near create-account contract.scxtt.testnet --masterAccount scxtt.testnet --initialBalance 4
```

Compile contract:
`cargo build --all --target wasm32-unknown-unknown --release`

Deploy contract:
`near deploy contract.scxtt.testnet --wasmFile target/wasm32-unknown-unknown/release/contract.wasm`

Check account state:
`near state contract.scxtt.testnet`

Check contract state:
`near view-state contract.scxtt.testnet --finality final`

change_string_vec:
`near call contract.scxtt.testnet change_string_vec '{"string": "stuff", "number": 2}' --accountId scxtt.testnet`
	
change_string:
`near call contract.scxtt.testnet change_string '{"string": "good morning"}' --accountId scxtt.testnet`
get_string:
`near view contract.scxtt.testnet get_string '{}'`
	
vector_push:
`near call contract.scxtt.testnet vector_push '{"number": 128}' --accountId scxtt.testnet`
get_vector_index:
`near view contract.scxtt.testnet get_vector_index '{"index": 0}'`
	
map_insert:
`near call contract.scxtt.testnet map_insert '{"key": "account 2", "value": "0x1234"}' --accountId scxtt.testnet`
get_map_value:
`near view contract.scxtt.testnet get_map_value '{"key": "account 2"}'`