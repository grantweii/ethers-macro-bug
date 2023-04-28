use ethers::contract::abigen;
use rug::Float;

abigen!(
    ChainlinkPriceFeedV2,
    "./src/ChainlinkPriceFeedV2.json"
);  

fn main() {
    println!("Hello, world!");
}
