use ccxt_binance::{Api, client::{Keys, Config}, ChartInterval};

use futures::{FutureExt, TryFutureExt};
use tokio::runtime::current_thread::run;

const SYMBOL: &str = "BNBBTC";

fn main() {
    let api = Api::with_keys(Keys::new(
        Some("Nfsp45SPC39aKz8ixzgwWW1Ng843RwwcsbORRCsgNdMbWRvZX72syMwHmsrLUIWh".to_string()),
        None,
    ));

    let future = async move {
        // let future = api.depth(SYMBOL, None);
        let future = api.trades(SYMBOL, None);
        // let future = api.historical_trades(SYMBOL, None, None);
        // let future = api.agg_trades(SYMBOL, None, None, None, None);
        // let future = api.klines(SYMBOL, ChartInterval::Minute1, None, None, None);
        // let future = api.avg_price(SYMBOL);
        // let future = api.ticker_24hr(SYMBOL);
        // let future = api.ticker_24hr_all();
        // let future = api.ticker_price(SYMBOL);
        // let future = api.ticker_price_all();
        // let future = api.ticker_book(SYMBOL);
        // let future = api.ticker_book_all();
        // let future = api.user_data_stream();
        match future.await {
            Ok(answer) => println!("Answer: {:#?}", answer),
            Err(e) => println!("Error: {:?}", e),
        }
    };
    run(future.unit_error().boxed_local().compat())
}
