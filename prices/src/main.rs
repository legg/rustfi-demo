use ethers_core::types::{Address, BlockNumber, H160};
use ethers_providers::{Http, Middleware, Provider};
use std::{
    str::FromStr,
    sync::Arc,
};
use uniswap_v2_bindings::uniswap_v2_pair::UniswapV2Pair;
pub use bigdecimal::{
    self,
    num_bigint::{self, BigInt},
    BigDecimal,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {

    // setup ethers-rs client
    let url = std::env::var("RPC_URL").unwrap_or("http://localhost:8545/".to_string());
    let client = Arc::new(Provider::<Http>::try_from(url)?);

    // usdc/weth address
    let address = Address::from_str("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc")?; // created at block  10008355

    // uniswap v2 bindings Pair contract
    let pair_access = UniswapV2Pair::new(address, client.clone());

    // Use ethers client to get block
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap();
    let block_height = i64::try_from(
        last_block
            .number
            .expect("block doesn't have a number")
            .as_u64(),
    )?;

    // function call to get reserves
    let (reserve_0, reserve_1, _last_block_timestamp) = pair_access
        .get_reserves()
        .block(last_block.hash.expect("block doesn't have a hash"))
        .call()
        .await?;

    // you can get this from the erc20
    let decimals_0: u32 = 6;
    let decimals_1: u32 = 18;

    // use decimals and reserves to compute price
    let decimal_diff = i64::from(decimals_0.abs_diff(decimals_1));
    let scale = BigDecimal::new(BigInt::from(1), -decimal_diff);
    let reserve_0_dec = BigDecimal::from(BigInt::from(reserve_0));
    let reserve_1_dec = BigDecimal::from(BigInt::from(reserve_1));

    let price = &scale * &reserve_0_dec / &reserve_1_dec;

    println!("USDC/WETH Price: {:?}", price);
    Ok(())
}
