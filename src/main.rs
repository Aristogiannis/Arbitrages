mod binance_api;
mod identify_arbitrage;

extern crate plotters;
extern crate petgraph;
extern crate serde_json;

use binance_api::get_prices;
use identify_arbitrage::identify_arbitrage;

fn main() {
    let prices = get_prices();
    
    //print!("{:?}", prices);

    identify_arbitrage( prices.unwrap());
    
}
