cargo init
forge init contracts --no-commit
forge install --root contracts/ uniswap/v2-core --no-commit
forge build --root contracts/ -c lib/v2-core/contracts/
touch contracts/src/a_file.sol ** do this to make bind work **
forge bind --root contracts -o ../contracts/out -b ./uniswap-v2-bindings --crate-name uniswap-v2-bindings
```Cargo.toml
[dependencies]
uniswap-v2-bindings = { path = "../uniswap-v2-bindings" }
```

cargo run
Use remappings 

