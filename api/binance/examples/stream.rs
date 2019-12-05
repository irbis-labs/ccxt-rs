use futures::{FutureExt, StreamExt, TryFutureExt};

use ccxt_binance::{Api, client::Keys, Error};
use tokio::runtime::current_thread::run;

fn main() {
    let api = Api::with_keys(Keys::new(
        None,
        None,
    ));
    let future = async move {
        let res = async move {
            api.ws_diff_depth("bnbbtc")
                .await?
                .for_each(|e| async move {
                    println!("{:?}", e);
                })
                .map(Ok::<(), Error>)
                .await
        };
        println!("Execution stopped with: {:?}", res.await);
    };
    run(future.unit_error().boxed_local().compat())
}
